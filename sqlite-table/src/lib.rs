use sqlite_decoder::btree;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum Schema {
    Table(Table),
    Index(Index),
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub sql: String,
    pub root_page: u32,
}

#[derive(Debug)]
pub struct PageWithRowidRange {
    pub range: RangeInclusive<u64>,
    pub index: u32,
}

impl Table {
    pub fn list_pages(&self, db: &sqlite_types::Db) -> Vec<PageWithRowidRange> {
        let page = db.pages.get(&self.root_page).unwrap();
        let res = sqlite_decoder::btree::decode(&db.header.text_encoding, page).unwrap();

        let mut page_list = Vec::new();

        let mut prev_rowid = None;
        for cell in res.cells {
            match cell {
                sqlite_decoder::btree::Cell::TableBTreeInteriorCell(cell) => {
                    let start = prev_rowid.unwrap_or_default();
                    let end = cell.rowid;

                    page_list.push(PageWithRowidRange {
                        range: RangeInclusive::new(start, end),
                        index: cell.left_child_page,
                    });
                    // TODO: can an Interior btree point to another interior btree?

                    prev_rowid = Some(end);
                }
                _ => unimplemented!(),
            }
        }

        page_list
    }
}

#[derive(Debug)]
pub struct Index {
    pub name: String,
    pub sql: String,
    pub tbl_name: String,
    pub root_page: u32,
}

/// Decodes SQLite schema table
/// The table is always located at page 1 (after the db3 header)
pub fn decode_sqlite_schema(db: &sqlite_types::Db) -> HashMap<String, Schema> {
    let page = db.pages.get(&1).unwrap();

    let enc = &db.header.text_encoding;
    let btree = btree::decode_first_page(enc, page).unwrap();

    let mut schemas = HashMap::new();

    for cell in btree.cells {
        match cell {
            btree::Cell::TableBTreeLeafCell(leaf) => {
                let record_type = leaf.records[0].as_string();
                let name = leaf.records[1].as_string();
                let tbl_name = leaf.records[2].as_string();
                let root_page = leaf.records[3].as_int() as u32;
                let sql = leaf.records[4].as_string();

                let schema = if record_type == "table" {
                    Schema::Table(Table {
                        name: name.clone(),
                        root_page,
                        sql,
                    })
                } else {
                    Schema::Index(Index {
                        name: name.clone(),
                        root_page,
                        sql,
                        tbl_name,
                    })
                };

                schemas.insert(name, schema);
            }
            _ => unreachable!(),
        }
    }

    schemas
}
