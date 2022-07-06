//! https://www.sqlite.org/fileformat.html

use nom::bytes::complete::take;
use nom::IResult;
use sqlite_types::{BtreeCell, BtreeCellTableLeaf, BtreePage, Db, DbHeader, MAGIC_STRING};
use vlq::ReadVlqExt;

type BoxError = Box<dyn std::error::Error>;

#[derive(Clone)]
pub struct ParsingContext<'a> {
    input: &'a [u8],
    /// Copy of the original input for data offset
    /// TODO: remove if possible?
    original_input: Vec<u8>,
}

fn read_vu64(input: &[u8]) -> IResult<&[u8], u64> {
    let mut data = std::io::Cursor::new(input.to_vec());
    println!("data {:?}", data.position());
    let x: u64 = data.read_vlq().unwrap();
    println!("data {:?}", data.position());

    Ok((input, x))
}

fn read_u32(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, value) = take(4usize)(input)?;
    Ok((input, u32::from_be_bytes(value.try_into().unwrap())))
}

fn read_u16(input: &[u8]) -> IResult<&[u8], u16> {
    let (input, value) = take(2usize)(input)?;
    Ok((input, u16::from_be_bytes(value.try_into().unwrap())))
}

fn read_u8(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, value) = take(1usize)(input)?;
    Ok((input, u8::from_be_bytes(value.try_into().unwrap())))
}

pub fn decode<'a>(input: &'a [u8]) -> Result<Db, BoxError> {
    let ctx = ParsingContext {
        input,
        original_input: input.clone().to_vec(),
    };
    match decode_db(ctx) {
        Ok((_, db)) => Ok(db),
        Err(err) => Err(format!("failed to decode: {}", err).into()),
    }
}

fn decode_db<'a, 'b>(ctx: ParsingContext<'a>) -> IResult<&'a [u8], Db> {
    let (input, input_header) = take(100usize)(ctx.input)?;
    let (_, header) = decode_header_inner(&input_header)?;
    let (input, btree_page) = decode_btree_page(ParsingContext {
        input,
        original_input: ctx.original_input.clone(),
    })?;
    Ok((input, Db { header, btree_page }))
}

fn decode_btree_page<'a, 'b>(ctx: ParsingContext<'a>) -> IResult<&'a [u8], BtreePage> {
    let (input, page_type) = read_u8(ctx.input)?;
    let (input, first_freeblock) = read_u16(input)?;
    let (input, num_cells) = read_u16(input)?;
    let (input, start_cell_content_area) = read_u16(input)?;
    let (input, num_frag_free_bytes) = read_u8(input)?;

    let (input, right_most_ptr) = if page_type == 2 || page_type == 5 {
        read_u16(input)?
    } else {
        (input, 0)
    };

    let mut cells = Vec::with_capacity(num_cells as usize);
    let mut input = input;
    for _ in 0..num_cells {
        let ret = decode_btree_cell(ctx.clone(), &page_type)?;
        input = ret.0;
        cells.push(ret.1);
    }

    Ok((
        input,
        BtreePage {
            page_type,
            first_freeblock,
            num_cells,
            start_cell_content_area,
            num_frag_free_bytes,
            right_most_ptr,
            cells,
        },
    ))
}

fn decode_btree_cell<'a>(
    ctx: ParsingContext<'a>,
    parent_page_type: &'_ u8,
) -> IResult<&'a [u8], BtreeCell> {
    assert_eq!(*parent_page_type, 13u8);
    let (input, offset) = read_u16(ctx.input)?;
    println!("offset {:?}", offset);

    let data = if (offset as usize) < ctx.original_input.len() {
        println!("{:?}", ctx.original_input[offset as usize..].to_vec());

        let (_, data) =
            decode_btree_cell_table_leaf(ctx.original_input[offset as usize..].to_vec()).unwrap();
        Some(data)
    // FIXME: remove unwrap
    } else {
        None
    };

    Ok((
        input,
        BtreeCell {
            offset,
            data: data.clone(),
        },
    ))
}

fn decode_btree_cell_table_leaf(input: Vec<u8>) -> IResult<(), BtreeCellTableLeaf> {
    // FIXME: remove unwrap
    let (input, len_payload) = read_vu64(&input).unwrap();
    let (input, row_id) = read_vu64(input).unwrap();
    Ok((
        (),
        BtreeCellTableLeaf {
            len_payload,
            row_id,
        },
    ))
}

pub fn decode_header(input: &[u8]) -> Result<DbHeader, BoxError> {
    match decode_header_inner(input) {
        Ok((_, header)) => Ok(header),
        Err(err) => Err(format!("failed to decode: {}", err).into()),
    }
}

fn decode_header_inner(input: &[u8]) -> IResult<&[u8], DbHeader> {
    let (input, magic_string) = take(16usize)(input)?;

    if magic_string != MAGIC_STRING {
        // FIXME: return error
        panic!("unsupported file format");
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
    let (input, text_encoding) = read_u32(input)?;
    let (input, user_version) = read_u32(input)?;
    let (input, vaccum_mode) = read_u32(input)?;
    let (input, app_id) = read_u32(input)?;
    let (input, _reserved) = take(20usize)(input)?;
    let (input, version_valid_for) = read_u32(input)?;
    let (input, sqlite_version) = read_u32(input)?;

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
