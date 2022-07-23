use std::env::args;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    let db_filename = &args[1];
    let wal_filename = &args[2];

    let db_contents = fs::read(db_filename)?;
    let wal_contents = fs::read(wal_filename)?;

    let mut db = sqlite_decoder::db::decode(&db_contents).unwrap();
    let wal = sqlite_decoder::wal::decode(&wal_contents).unwrap();

    if db.header.page_size as u32 != wal.header.page_size {
        panic!(
            "Error: page_size mismatch between WAL ({}) and DB ({}).",
            wal.header.page_size, db.header.page_size
        );
    }

    for frame in wal.frames {
        // FIXME: commit frames?
        assert_eq!(wal.header.page_size as usize, frame.data.len());

        // Page numbers are 1 indexed

        if frame.header.page_number == 1 {
            // Update the database header
            let new_header = sqlite_decoder::db::decode_header(&frame.data).unwrap();
            println!("replace header: {:?}", new_header);
            db.header = new_header;
        } else if let Some(page) = db.pages.get_mut(&frame.header.page_number) {
            *page = frame.data;
            println!("replace page: {}", frame.header.page_number);
        } else {
            db.pages.insert(frame.header.page_number, frame.data);
            db.header.db_size += 1;
            println!("create new page: {}", frame.header.page_number);
        }

        if frame.header.db_size_after_commit != 0
            && (frame.header.db_size_after_commit as usize) < db.pages.len() - 1
        {
            todo!("truncate");
        }
    }

    let bytes = sqlite_encoder::db::encode(&db).unwrap();

    let out_filename = format!("{}.out.db3", db_filename);
    println!("out: {}", out_filename);
    let mut file = File::create(out_filename)?;
    file.write_all(&bytes)?;

    Ok(())
}
