use std::env::args;
use std::fs;

fn main() {
    let filename = args().last().unwrap();
    let contents = fs::read(filename).unwrap();

    println!("{:#?}", sqlite_decoder::db::decode(&contents));
}
