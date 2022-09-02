use pretty_hex::pretty_hex;
use std::env::args;
use std::fs;

fn main() {
    let filename = args().last().unwrap();
    let contents = fs::read(filename).unwrap();

    let db = sqlite_decoder::db::decode(&contents).unwrap();

    println!("Header: {:?}", db.header);
    println!("Pages:");
    for i in 0..db.header.db_size {
        // Page number are 1 indexed and 1 is the db header
        let page_number = i + 1;

        if let Some(page) = db.pages.get(&page_number) {
            println!("page {} data {} bytes.", i, page.len());
            println!("{}", pretty_hex(&page));
        } else {
            println!("page {} data empty.", i);
        }
    }
}
