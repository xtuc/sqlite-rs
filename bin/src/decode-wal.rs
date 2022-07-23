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
        println!("{:?}.", frame.header);
        println!("{}", pretty_hex(&frame.data));
    }
}
