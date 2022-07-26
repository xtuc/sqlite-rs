use pretty_hex::pretty_hex;
use std::env::args;
use std::fs;

fn main() {
    let filename = args().last().unwrap();
    let contents = fs::read(filename).unwrap();

    let wal = sqlite_decoder::wal::decode(&contents).unwrap();
    println!("Header: {:?}", wal.header);
    println!("Frames:");
    for frame in wal.frames {
        println!("{:?}", frame.header);
        if frame.header.page_number == 1 {
            let db_header = sqlite_decoder::db::decode_header(&frame.data).unwrap();
            println!("new header: {:?}", db_header);
        }
        println!("{}", pretty_hex(&frame.data));
    }
}
