use std::env::args;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    let filename = &args[1];

    let contens = fs::read(filename)?;

    let wal = sqlite_decoder::wal::decode(&contens).unwrap();

    // Values taken from a newly created sqlite3 database
    let db_header = sqlite_types::DbHeader {
        page_size: wal.header.page_size,
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
        text_encoding: sqlite_types::TextEncoding::UTF8,
        user_version: 0,
        vaccum_mode: 0,
        app_id: 0,
        version_valid_for: 1,
        sqlite_version: sqlite_types::SQLITE_3_37_2_VERSION,
    };
    let db = sqlite_wal::to_db(&db_header, &wal).unwrap();

    let bytes = sqlite_encoder::db::encode(&db).unwrap();

    let out_filename = format!("{}.out.db3", filename);
    println!("out: {}", out_filename);
    let mut file = File::create(out_filename)?;
    file.write_all(&bytes)?;

    Ok(())
}
