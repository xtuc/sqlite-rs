///! Module to manipulate WAL files
use std::collections::HashMap;
use std::io::Write;

type Error = Box<dyn std::error::Error + Send + Sync>;

/// Moving content from WAL to a database (called "backfilling").
/// Arguments:
/// - `db`: database to backfill
/// - `wal`: WAL to apply
/// Warning: risks of corruption if used on a live database.
pub fn backfill(db: &mut sqlite_types::Db, wal: &sqlite_types::Wal) -> Result<(), Error> {
    if db.header.page_size as u32 != wal.header.page_size {
        return Err(format!(
            "Error: page_size mismatch between WAL ({}) and DB ({}).",
            wal.header.page_size, db.header.page_size
        )
        .into());
    }

    for frame in &wal.frames {
        assert_eq!(wal.header.page_size as usize, frame.data.len());

        if let Some(page) = db.pages.get_mut(&frame.header.page_number) {
            if frame.header.page_number == 1 {
                // The first page (page are 1 indexed) is the header
                let new_header = sqlite_decoder::db::decode_header(&frame.data).unwrap();
                db.header = new_header;
            }

            *page = frame.data.clone();
        } else {
            db.pages
                .insert(frame.header.page_number, frame.data.clone());
            db.header.db_size += 1;
        }
    }

    Ok(())
}

pub fn backfill_bytes(wal: &sqlite_types::Wal, db_bytes: &mut Vec<u8>) -> Result<(), Error> {
    let db_header = sqlite_decoder::db::decode_header(&db_bytes)
        .map_err(|err| format!("failed to decode database header: {}", err))?;

    if db_header.page_size as u32 != wal.header.page_size {
        return Err(format!(
            "Error: page_size mismatch between WAL ({}) and DB ({}).",
            wal.header.page_size, db_header.page_size
        )
        .into());
    }

    for frame in &wal.frames {
        assert_eq!(wal.header.page_size as usize, frame.data.len());

        let db_offset = (frame.header.page_number as usize - 1) * wal.header.page_size as usize;
        let end = db_offset + wal.header.page_size as usize;

        if end > db_bytes.len() {
            // Writing a new page requires growing the database
            db_bytes.resize(end, 0);
        }

        let wrote = (&mut db_bytes[db_offset..end])
            .write(&frame.data)
            .map_err(|err| format!("failed to write: {}", err))?;
        assert_eq!(wrote, wal.header.page_size as usize);
    }

    Ok(())
}

/// Turn a WAL into a database
pub fn to_db(
    db_header: &sqlite_types::DbHeader,
    wal: &sqlite_types::Wal,
) -> Result<sqlite_types::Db, Error> {
    let mut pages = HashMap::new();

    // Write first page
    {
        let header_bytes =
            sqlite_encoder::db::encode_header(&db_header).map_err(|err| -> Error {
                format!("failed to encode database header: {}", err).into()
            })?;
        let mut first_page = vec![0u8; db_header.page_size as usize];
        (&mut first_page[0..100])
            .write(&header_bytes)
            .map_err(|err| format!("failed to write header: {}", err))?;

        pages.insert(1, first_page);
    }

    let mut db = sqlite_types::Db {
        header: db_header.clone(),
        pages,
    };
    backfill(&mut db, wal)?;

    Ok(db)
}

pub fn merge(wal1: &mut sqlite_types::Wal, wal2: &sqlite_types::Wal) -> Result<(), Error> {
    wal1.frames.extend(wal2.frames.clone());

    // FIXME: rewrite the Salts to be consistent
    // *wal1 = wal1
    //     .rewrite_salt_1(wal1.header.salt_1)
    //     .rewrite_salt_2(wal1.header.salt_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn open_db(db: &sqlite_types::Db, f: Box<dyn Fn(rusqlite::Connection)>) {
        let bytes = sqlite_encoder::db::encode(db).unwrap();

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(&bytes).unwrap();
        file.flush().unwrap();

        let conn = rusqlite::Connection::open(file.path()).unwrap();
        f(conn);

        file.close().unwrap();
    }

    fn table_list(conn: &rusqlite::Connection) -> Vec<String> {
        let mut stmt = conn.prepare("pragma table_list;").unwrap();
        let rows = stmt.query_map([], |row| row.get(1)).unwrap();

        let mut list = Vec::new();
        for row in rows {
            list.push(row.unwrap());
        }
        list
    }

    fn pragma<T: rusqlite::types::FromSql>(conn: &rusqlite::Connection, name: &str) -> T {
        let mut stmt = conn.prepare(&format!("pragma {};", name)).unwrap();
        stmt.query_row([], |row| row.get::<usize, T>(0)).unwrap()
    }

    #[test]
    fn it_converts_wal_to_db() {
        let db_header = sqlite_types::DbHeader {
            page_size: 4096,
            file_format_write_version: 2,
            file_format_read_version: 2,
            max_embedded_payload_frac: 64,
            min_embedded_payload_frac: 32,
            leaf_payload_frac: 32,
            file_change_counter: 1,
            db_size: 1,
            page_num_first_freelist: 0,
            page_count_freelist: 0,
            schema_cookie: 1,
            schema_format_number: 4,
            default_page_cache_size: 0,
            page_num_largest_root_btree: 0,
            text_encoding: 1,
            user_version: 0,
            vaccum_mode: 0,
            app_id: 0,
            version_valid_for: 1,
            sqlite_version: sqlite_types::SQLITE_3_37_2_VERSION,
        };

        let wal = include_bytes!("../test/create-test-table.wal");
        let wal = sqlite_decoder::wal::decode(wal).unwrap();
        let db = to_db(&db_header, &wal).unwrap();

        open_db(
            &db,
            Box::new(move |conn| {
                let tables = table_list(&conn);
                assert!(
                    tables.contains(&"test".to_owned()),
                    "`test` table was not found; meaning WAL wasn't applied correctly."
                );
            }),
        );
    }

    #[test]
    fn it_applies_wal_on_top_of_db() {
        let db = include_bytes!("../test/existing.db3");
        let mut db = sqlite_decoder::db::decode(db).unwrap();

        {
            let wal = include_bytes!("../test/create-test-table.wal");
            let wal = sqlite_decoder::wal::decode(wal).unwrap();

            backfill(&mut db, &wal).unwrap();

            open_db(
                &db,
                Box::new(move |conn| {
                    let tables = table_list(&conn);
                    assert!(
                        tables.contains(&"test".to_owned()),
                        "`test` table was not found; WAL wasn't applied correctly."
                    );
                }),
            );
        }

        {
            let wal = include_bytes!("../test/create-test-and-test2-table.wal");
            let wal = sqlite_decoder::wal::decode(wal).unwrap();

            backfill(&mut db, &wal).unwrap();

            open_db(
                &db,
                Box::new(move |conn| {
                    let tables = table_list(&conn);
                    assert!(
                        tables.contains(&"test".to_owned()),
                        "`test` table was not found."
                    );
                    assert!(
                        tables.contains(&"test2".to_owned()),
                        "`test2` table was not found; second WAL wasn't applied correctly."
                    );
                }),
            );
        }

        {
            let wal = include_bytes!("../test/test-data.wal");
            let wal = sqlite_decoder::wal::decode(wal).unwrap();

            backfill(&mut db, &wal).unwrap();

            open_db(
                &db,
                Box::new(move |conn| {
                    let tables = table_list(&conn);
                    assert!(
                        tables.contains(&"test".to_owned()),
                        "`test` table was not found."
                    );

                    let mut stmt = conn.prepare("select count(*) from test;").unwrap();
                    let count: usize = stmt.query_row([], |row| row.get(0)).unwrap();
                    assert_eq!(count, 65);

                    let page_count: usize = pragma(&conn, "page_count");
                    assert_eq!(page_count, 19);
                }),
            );
        }

        {
            let wal = include_bytes!("../test/delete-test-table.wal");
            let wal = sqlite_decoder::wal::decode(wal).unwrap();

            backfill(&mut db, &wal).unwrap();

            open_db(
                &db,
                Box::new(move |conn| {
                    let tables = table_list(&conn);
                    assert!(
                        !tables.contains(&"test".to_owned()),
                        "`test` table was found; WAL wasn't applied correctly"
                    );

                    let page_count: usize = pragma(&conn, "page_count");
                    assert_eq!(page_count, 18);
                }),
            );
        }

        {
            let wal = include_bytes!("../test/vacuum.wal");
            let wal = sqlite_decoder::wal::decode(wal).unwrap();

            backfill(&mut db, &wal).unwrap();

            open_db(
                &db,
                Box::new(move |conn| {
                    let page_count: usize = pragma(&conn, "page_count");
                    assert_eq!(page_count, 1);
                }),
            );
        }
    }
}
