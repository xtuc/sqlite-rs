use pretty_hex::pretty_hex;
use std::env::args;
use std::fs;

fn main() {
    let filename = args().last().unwrap();
    let contents = fs::read(filename).unwrap();

    let db = sqlite_decoder::db::decode(&contents).unwrap();

    println!("Header: {:?}", db.header);
    println!("Pages:");
    for (i, page) in db.pages {
        println!("page {} data {} bytes.", i, page.len());
        println!("{}", pretty_hex(&page));
    }
}
