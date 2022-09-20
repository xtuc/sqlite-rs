use sqlite_decoder::btree;
use std::collections::HashMap;
use std::ops::RangeInclusive;

pub type Schemas = HashMap<String, Schema>;
type BoxError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum Schema {
    Table(Table),
    Index(Index),
}

#[derive(Debug, Clone)]
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
    pub fn list_pages(&self, db: &sqlite_types::Db) -> Result<Vec<PageWithRowidRange>, BoxError> {
        let page = db.pages.get(&self.root_page).ok_or(format!(
            "table root page ({}) not found in the database",
            self.root_page
        ))?;
        let res = sqlite_decoder::btree::decode(&db.header.text_encoding, page)
            .map_err(|err| format!("failed to decode B-tree: {}", err))?;

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

        Ok(page_list)
    }
}

#[derive(Debug)]
pub struct Index {
    pub name: String,
    pub sql: String,
    pub tbl_name: String,
    pub root_page: u32,
}

pub fn find_table_by_root(rootpage: usize, schemas: &Schemas) -> Option<Table> {
    let mut table = None;

    for (_, schema) in schemas {
        match schema {
            Schema::Table(schema) => {
                if schema.root_page as usize == rootpage {
                    table = Some(schema.clone());
                }
            }
            _ => {}
        }
    }

    table
}

/// Decodes SQLite schema table
/// The table is always located at page 1 (after the db3 header)
pub fn decode_sqlite_schema(db: &sqlite_types::Db) -> Result<Schemas, BoxError> {
    let root = 1;
    let page = db.pages.get(&root).ok_or(format!(
        "table root page ({}) not found in the database",
        root
    ))?;

    let enc = &db.header.text_encoding;
    let btree = btree::decode_first_page(enc, page)
        .map_err(|err| format!("failed to decode B-tree: {}", err))?;

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

    Ok(schemas)
}
