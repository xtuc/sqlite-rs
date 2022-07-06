use std::env::args;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    let first_filename = &args[1];
    let second_filename = &args[2];

    let first_contents = fs::read(first_filename).unwrap();
    let second_contents = fs::read(second_filename).unwrap();

    let mut first_wal = sqlite_decoder::wal::decode(&first_contents).unwrap();
    let second_wal = sqlite_decoder::wal::decode(&second_contents).unwrap();

    first_wal.frames.extend(second_wal.frames);
    let first_wal = first_wal.rewrite_salt_1(1).rewrite_salt_2(2);

    let bytes = sqlite_encoder::wal::encode(first_wal).unwrap();

    println!("out: ./out.wal");
    let mut file = File::create("out.wal")?;
    file.write_all(&bytes)?;

    Ok(())
}
