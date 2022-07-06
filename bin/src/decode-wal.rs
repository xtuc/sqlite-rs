use std::env::args;
use std::fs;

fn main() {
    let filename = args().last().unwrap();
    let contents = fs::read(filename).unwrap();

    let wal = sqlite_decoder::wal::decode(&contents).unwrap();
    println!("wal {:?}", wal);
    for frame in wal.frames {
        println!("frame {:?}", sqlite_decoder::db::decode(&frame.data));
    }
}
