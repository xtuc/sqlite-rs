//! https://www.sqlite.org/fileformat.html
use crate::util::{read_u16, read_u32, read_u8};
use crate::IResult;
use crate::ParserError;
use nom::bytes::complete::take;
use sqlite_types::{Db, DbHeader, TextEncoding, MAGIC_STRING};
use std::collections::HashMap;

type BoxError = Box<dyn std::error::Error>;

pub fn decode<'a>(input: &'a [u8]) -> Result<Db, BoxError> {
    match decode_db(input) {
        Ok((_, db)) => Ok(db),
        Err(err) => Err(format!("failed to decode: {}", err).into()),
    }
}

fn decode_db<'a, 'b>(input: &'a [u8]) -> IResult<&'a [u8], Db> {
    let mut pages = HashMap::new();

    let (input, input_header) = take(100usize)(input)?;
    let (_, header) = decode_header_inner(&input_header)?;

    // Eat align to page size and discard the bytes
    let (input, bytes) = take(header.page_size - 100)(input)?;

    // First page contains the header and is page aligned
    let first_page = [input_header, bytes].concat();
    pages.insert(1, first_page);

    // The remaining bytes should be pages and the number should match the
    // db_size in the header
    assert_eq!(
        input.len(),
        header.page_size as usize * (header.db_size as usize - 1)
    );

    let page_count = input.len() / header.page_size as usize;

    let mut input = input;
    for i in 1..=page_count {
        let ret = take(header.page_size)(input)?;
        input = ret.0;

        // Page number are 1 indexed and 1 is the db header
        let page_number = i + 1;
        pages.insert(page_number as u32, ret.1.to_owned());
    }

    assert_eq!(pages.len(), header.db_size as usize);
    Ok((input, Db { header, pages }))
}

pub fn decode_header(input: &[u8]) -> Result<DbHeader, BoxError> {
    match decode_header_inner(input) {
        Ok((_, header)) => Ok(header),
        Err(err) => Err(format!("failed to decode: {}", err).into()),
    }
}

fn decode_text_encoding(input: &[u8]) -> IResult<&[u8], TextEncoding> {
    let (input, t) = read_u32(input)?;

    use TextEncoding::*;
    let enc = match t {
        0 => Unspecified,
        1 => UTF8,
        2 => UTF16le,
        3 => UTF16be,
        e => {
            return Err(nom::Err::Failure(ParserError(format!(
                "unsupported text encoding: {}",
                e
            ))))
        }
    };

    Ok((input, enc))
}

fn decode_header_inner(input: &[u8]) -> IResult<&[u8], DbHeader> {
    let (input, magic_string) = take(16usize)(input)?;
    if magic_string != MAGIC_STRING {
        return Err(nom::Err::Failure(ParserError(format!(
            "magic string not found, got: {:?}",
            magic_string
        ))));
    }

    let (input, page_size) = read_u16(input)?;
    let (input, file_format_write_version) = read_u8(input)?;
    let (input, file_format_read_version) = read_u8(input)?;
    let (input, _reserved) = take(1usize)(input)?;
    let (input, max_embedded_payload_frac) = read_u8(input)?;
    let (input, min_embedded_payload_frac) = read_u8(input)?;
    let (input, leaf_payload_frac) = read_u8(input)?;
    let (input, file_change_counter) = read_u32(input)?;
    let (input, db_size) = read_u32(input)?;
    let (input, page_num_first_freelist) = read_u32(input)?;
    let (input, page_count_freelist) = read_u32(input)?;
    let (input, schema_cookie) = read_u32(input)?;
    let (input, schema_format_number) = read_u32(input)?;
    let (input, default_page_cache_size) = read_u32(input)?;
    let (input, page_num_largest_root_btree) = read_u32(input)?;
    let (input, text_encoding) = decode_text_encoding(input)?;
    let (input, user_version) = read_u32(input)?;
    let (input, vaccum_mode) = read_u32(input)?;
    let (input, app_id) = read_u32(input)?;
    let (input, _reserved) = take(20usize)(input)?;
    let (input, version_valid_for) = read_u32(input)?;
    let (input, sqlite_version) = read_u32(input)?;

    let page_size = if page_size == 1 {
        65536
    } else {
        page_size as u32
    };

    Ok((
        input,
        DbHeader {
            page_size,
            file_format_write_version,
            file_format_read_version,
            max_embedded_payload_frac,
            min_embedded_payload_frac,
            leaf_payload_frac,
            file_change_counter,
            db_size,
            page_num_first_freelist,
            page_count_freelist,
            schema_cookie,
            schema_format_number,
            default_page_cache_size,
            page_num_largest_root_btree,
            text_encoding,
            user_version,
            vaccum_mode,
            app_id,
            version_valid_for,
            sqlite_version,
        },
    ))
}
