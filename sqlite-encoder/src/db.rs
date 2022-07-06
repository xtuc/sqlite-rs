use sqlite_types::{DbHeader, MAGIC_STRING};

type BoxError = Box<dyn std::error::Error>;

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

fn write_header(writer: &mut Vec<u8>, header: &DbHeader) -> Result<(), BoxError> {
    write_bytes(writer, MAGIC_STRING);
    write_u16(writer, header.page_size);
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
    write_u32(writer, header.text_encoding);
    write_u32(writer, header.user_version);
    write_u32(writer, header.vaccum_mode);
    write_u32(writer, header.app_id);
    write_bytes(writer, &[0; 20]);
    write_u32(writer, header.version_valid_for);
    write_u32(writer, header.sqlite_version);

    Ok(())
}
