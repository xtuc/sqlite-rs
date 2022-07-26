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

    sqlite_wal::backfill(&mut db, &wal).unwrap();

    let bytes = sqlite_encoder::db::encode(&db).unwrap();

    let out_filename = format!("{}.out.db3", db_filename);
    println!("out: {}", out_filename);
    let mut file = File::create(out_filename)?;
    file.write_all(&bytes)?;

    Ok(())
}
