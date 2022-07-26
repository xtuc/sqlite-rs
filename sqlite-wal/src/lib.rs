///! Module to manipulate WAL files

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

        // Page numbers are 1 indexed
        if frame.header.page_number == 1 {
            // Update the database header
            let new_header = sqlite_decoder::db::decode_header(&frame.data).unwrap();
            println!("replace header: {:?}", new_header);
            db.header = new_header;
        } else if let Some(page) = db.pages.get_mut(&frame.header.page_number) {
            *page = frame.data.clone();
            println!("replace page: {}", frame.header.page_number);
        } else {
            db.pages
                .insert(frame.header.page_number, frame.data.clone());
            db.header.db_size += 1;
            println!("create new page: {}", frame.header.page_number);
        }

        if frame.header.db_size_after_commit != 0
            && (frame.header.db_size_after_commit as usize) < db.pages.len() - 1
        {
            todo!("truncate");
        }
    }

    Ok(())
}

pub fn merge(wal1: &mut sqlite_types::Wal, wal2: &sqlite_types::Wal) -> Result<(), Error> {
    wal1.frames.extend(wal2.frames.clone());

    // FIXME: rewrite the Salts to be consistent
    // *wal1 = wal1
    //     .rewrite_salt_1(wal1.header.salt_1)
    //     .rewrite_salt_2(wal1.header.salt_2);
    Ok(())
}
