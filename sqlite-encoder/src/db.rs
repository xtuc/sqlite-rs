use sqlite_types::{Db, DbHeader, TextEncoding, MAGIC_STRING};
use std::io::Write;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn write_bytes(writer: &mut Vec<u8>, value: &[u8]) {
    writer.extend_from_slice(value);
}

fn write_u16(writer: &mut Vec<u8>, value: u16) {
    writer.extend(value.to_be_bytes());
}

fn write_u32(writer: &mut Vec<u8>, value: u32) {
    writer.extend(value.to_be_bytes());
}

fn write_byte(writer: &mut Vec<u8>, value: u8) {
    writer.push(value);
}

pub fn encode_header(header: &DbHeader) -> Result<Vec<u8>, BoxError> {
    let mut buff = Vec::new();

    write_header(&mut buff, &header).map_err(|err| format!("failed to encode header: {}", err))?;

    Ok(buff)
}

pub fn encode(db: &Db) -> Result<Vec<u8>, BoxError> {
    let mut buff = Vec::new();

    let header_bytes = encode_header(&db.header)?;
    let mut first_page = db.pages.get(&1).ok_or("missing page 1")?.clone();

    (&mut first_page[0..100])
        .write(&header_bytes)
        .map_err(|err| format!("failed to write header: {}", err))?;
    buff.write(&first_page)
        .map_err(|err| format!("failed to write first page: {}", err))?;

    for i in 1..db.header.db_size {
        // Page number are 1 indexed and 1 is the db header
        let page_number = i + 1;

        if let Some(page) = db.pages.get(&page_number) {
            write_bytes(&mut buff, page);
        } else {
            // The page didn't exists, write an empty one
            write_bytes(&mut buff, &vec![0u8; db.header.page_size as usize]);
        }
    }

    Ok(buff)
}

fn write_header(writer: &mut Vec<u8>, header: &DbHeader) -> Result<(), BoxError> {
    let page_size = if header.page_size == 65536 {
        1u16
    } else {
        header.page_size as u16
    };

    write_bytes(writer, MAGIC_STRING);
    write_u16(writer, page_size);
    write_byte(writer, header.file_format_write_version);
    write_byte(writer, header.file_format_read_version);
    write_byte(writer, 0);
    write_byte(writer, header.max_embedded_payload_frac);
    write_byte(writer, header.min_embedded_payload_frac);
    write_byte(writer, header.leaf_payload_frac);
    write_u32(writer, header.file_change_counter);
    write_u32(writer, header.db_size);
    write_u32(writer, header.page_num_first_freelist);
    write_u32(writer, header.page_count_freelist);
    write_u32(writer, header.schema_cookie);
    write_u32(writer, header.schema_format_number);
    write_u32(writer, header.default_page_cache_size);
    write_u32(writer, header.page_num_largest_root_btree);
    write_text_encoding(writer, &header.text_encoding)?;
    write_u32(writer, header.user_version);
    write_u32(writer, header.vaccum_mode);
    write_u32(writer, header.app_id);
    write_bytes(writer, &[0; 20]);
    write_u32(writer, header.version_valid_for);
    write_u32(writer, header.sqlite_version);

    Ok(())
}

fn write_text_encoding(writer: &mut Vec<u8>, enc: &TextEncoding) -> Result<(), BoxError> {
    use TextEncoding::*;

    let v = match enc {
        Unspecified => 0,
        UTF8 => 1,
        UTF16le => 2,
        UTF16be => 3,
    };
    write_u32(writer, v);

    Ok(())
}
