use std::env::args;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    let db_filename = &args[1];
    let new_page_size = args[2].parse::<u32>().unwrap();

    let mut db_contents = fs::read(db_filename).unwrap();

    let mut header = sqlite_decoder::db::decode_header(&db_contents).unwrap();
    header.page_size = new_page_size;

    let bytes = sqlite_encoder::db::encode_header(&header).unwrap();
    (&mut db_contents[..100]).write(&bytes).unwrap();

    println!("out: {}.resized", db_filename);
    let mut file = File::create(format!("{}.resized", db_filename))?;
    file.write_all(&bytes)?;

    Ok(())
}
