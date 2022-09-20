mod constant;
pub use constant::Opcode;

#[derive(Debug)]
pub struct Explanation {
    pub table: Option<sqlite_table::Table>,
}

#[derive(Debug)]
pub struct Instr {
    pub addr: i32,
    pub comment: Option<String>,
    pub opcode: Opcode,
    pub p1: serde_json::Value,
    pub p2: serde_json::Value,
    pub p3: serde_json::Value,
    pub p4: serde_json::Value,
    pub p5: serde_json::Value,
}

pub fn explain(program: Vec<Instr>, schemas: &sqlite_table::Schemas) -> Explanation {
    let mut table = None;

    for instr in program {
        if matches!(
            instr.opcode,
            Opcode::OpenRead | Opcode::OpenWrite | Opcode::ReopenIdx
        ) {
            let id = match instr.p2 {
                serde_json::value::Value::Number(n) => n.as_u64().unwrap_or_default() as usize,
                _ => unreachable!(),
            };

            table = sqlite_table::find_table_by_root(id, schemas);
        }
    }

    Explanation { table }
}
