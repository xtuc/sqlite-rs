//! https://www.sqlite.org/fileformat.html

use crate::util;
use crate::IResult;
use crate::ParserError;
use nom::bytes::complete::take;
use sqlite_types::TextEncoding;

type BoxError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum Record {
    Null,
    Int8(i8),
    Int16(i16),
    Blob(Vec<u8>),
    Text(String),
}

impl Record {
    pub fn as_string(&self) -> String {
        match self {
            Self::Text(v) => v.clone(),
            Self::Null => "NULL".to_owned(),
            v => unreachable!("expected string, given {:?}", v),
        }
    }

    pub fn as_int(&self) -> usize {
        match self {
            Self::Int8(v) => *v as usize,
            Self::Int16(v) => *v as usize,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct InputContext<'a> {
    pub(crate) input: &'a [u8],
    pub(crate) original_input: Vec<u8>,
}

#[derive(Debug)]
pub enum Cell {
    // FIXME: Remove BTree in those names
    TableBTreeLeafCell(TableBTreeLeafCell),
    TableBTreeInteriorCell(TableBTreeInteriorCell),
    IndexBTreeLeafCell(IndexBTreeLeafCell),
    IndexBTreeInteriorCell(IndexBTreeInteriorCell),
}

#[derive(Debug)]
pub struct TableBTreeLeafCell {
    /// A varint which is the integer key, a.k.a. "rowid"
    rowid: u64,
    pub records: Vec<Record>,
    page_first_overflow: Option<u32>,
}

#[derive(Debug)]
pub struct TableBTreeInteriorCell {
    pub left_child_page: u32,
    pub rowid: u64,
}

#[derive(Debug)]
pub struct IndexBTreeLeafCell {}

#[derive(Debug)]
pub struct IndexBTreeInteriorCell {}

#[derive(Debug)]
pub struct BtreeHeader {
    page_type: PageType,
    start_first_freeblock: u16,
    cell_count: u16,
    start_cell_content_area: u16,
    fragmented_free_bytes_count: u8,
    right_most_pointer: Option<u32>,
}

#[derive(Debug)]
pub struct Btree {
    pub header: BtreeHeader,
    pub cells: Vec<Cell>,
}

#[derive(Debug)]
enum PageContent {
    Index,
    Table,
}

#[derive(Debug)]
enum PageType {
    Leaf(PageContent),
    Interior(PageContent),
}
impl PageType {
    fn header_size(&self) -> u8 {
        use PageType::*;
        match self {
            Leaf(_) => 8,
            Interior(_) => 12,
        }
    }

    fn is_interior(&self) -> bool {
        matches!(self, PageType::Interior(_))
    }
}

fn decode_page_type<'a>(input: InputContext<'a>) -> IResult<InputContext<'a>, PageType> {
    let (input, byte) = input.read_u8()?;
    let t = match byte {
        0x02 => PageType::Interior(PageContent::Index),
        0x05 => PageType::Interior(PageContent::Table),
        0x0a => PageType::Leaf(PageContent::Index),
        0x0d => PageType::Leaf(PageContent::Table),
        e => {
            return Err(nom::Err::Failure(ParserError(format!(
                "unsupported page type: {}",
                e
            ))))
        }
    };
    Ok((input, t))
}

fn decode_header<'a>(input: InputContext<'a>) -> IResult<InputContext<'a>, BtreeHeader> {
    let (input, page_type) = decode_page_type(input)?;
    let (input, start_first_freeblock) = input.read_u16()?;
    let (input, cell_count) = input.read_u16()?;
    let (input, start_cell_content_area) = input.read_u16()?;
    let (input, fragmented_free_bytes_count) = input.read_u8()?;

    let (input, right_most_pointer) = if page_type.is_interior() {
        let (input, right_most_pointer) = input.read_u32()?;
        (input, Some(right_most_pointer))
    } else {
        (input, None)
    };

    let header = BtreeHeader {
        page_type,
        start_first_freeblock,
        cell_count,
        start_cell_content_area,
        fragmented_free_bytes_count,
        right_most_pointer,
    };
    Ok((input, header))
}

fn decode_cell_pointers<'a>(
    header: &BtreeHeader,
    mut input: InputContext<'a>,
) -> IResult<InputContext<'a>, Vec<u16>> {
    let mut cell_pointers = Vec::with_capacity(header.cell_count as usize);

    for _ in 0..header.cell_count {
        let res = input.read_u16()?;
        input = res.0;
        cell_pointers.push(res.1);
    }

    Ok((input, cell_pointers))
}

fn decode_cell<'a>(
    enc: &TextEncoding,
    parent: &PageType,
    input: InputContext<'a>,
) -> IResult<InputContext<'a>, Cell> {
    let (input, cell) = match parent {
        PageType::Leaf(PageContent::Table) => {
            let (input, cell) = decode_table_leaf_cell(enc, input)?;
            (input, Cell::TableBTreeLeafCell(cell))
        }
        PageType::Interior(PageContent::Table) => {
            let (input, cell) = decode_table_interior_cell(input)?;
            (input, Cell::TableBTreeInteriorCell(cell))
        }
        e => {
            return Err(nom::Err::Failure(ParserError(format!(
                "unsupported cell with parent: {:?}",
                e
            ))))
        }
    };

    Ok((input, cell))
}

fn decode_table_interior_cell<'a>(
    input: InputContext<'a>,
) -> IResult<InputContext<'a>, TableBTreeInteriorCell> {
    let (input, left_child_page) = input.read_u32()?;
    let (input, rowid) = input.read_varint()?;

    Ok((
        input,
        TableBTreeInteriorCell {
            left_child_page,
            rowid,
        },
    ))
}

fn decode_table_leaf_cell<'a>(
    enc: &TextEncoding,
    input: InputContext<'a>,
) -> IResult<InputContext<'a>, TableBTreeLeafCell> {
    let (input, total_payload_size) = input.read_varint()?;
    let (input, rowid) = input.read_varint()?;
    // FIXME: using the total_payload_size to read only the part that doesn't
    // overflow?
    let (input, raw_payload) = input.read_bytes(total_payload_size as usize)?;
    let (_input, records) = decode_records(enc, raw_payload)?;

    // FIXME: implement overflow handling. How to detect if it overflowed?
    // FIXME: it's written in the spec
    // let (input, page_first_overflow) = input.read_u32()?;
    let page_first_overflow = None;

    Ok((
        input,
        TableBTreeLeafCell {
            rowid,
            records,
            page_first_overflow,
        },
    ))
}

fn decode_records<'a>(enc: &TextEncoding, input: &'a [u8]) -> IResult<&'a [u8], Vec<Record>> {
    let (input, (header_size, took)) = read_varint(input)?;

    // Header without the header size varint
    let header_input = &input[..header_size as usize - took];
    let (_input, columns) = decode_record_columns(header_input)?;

    let mut input = &input[header_size as usize - took..];

    let records = {
        let mut values = Vec::with_capacity(columns.len());

        for serial_type in columns {
            let res = decode_record_value(enc, serial_type, input)?;
            input = res.0;
            values.push(res.1);
        }

        values
    };

    Ok((input, records))
}

fn decode_record_columns<'a>(mut input: &'a [u8]) -> IResult<&'a [u8], Vec<u64>> {
    let mut columns = Vec::new();
    loop {
        let res = read_varint(input)?;
        input = res.0;
        columns.push(res.1 .0);

        if input.is_empty() {
            break;
        }
    }

    Ok((input, columns.to_owned()))
}

fn decode_record_value<'a>(
    enc: &TextEncoding,
    serial_type: u64,
    input: &'a [u8],
) -> IResult<&'a [u8], Record> {
    use Record::*;
    let (input, serial_type) = match serial_type {
        0 => (input, Null),
        1 => {
            let (input, value) = take(1usize)(input)?;
            (input, Int8(i8::from_be_bytes(value.try_into().unwrap())))
        }
        2 => {
            let (input, value) = take(2usize)(input)?;
            (input, Int16(i16::from_be_bytes(value.try_into().unwrap())))
        }
        v if v > 12 && v % 2 == 0 => {
            let size = (v - 12) / 2;
            let (input, bytes) = take(size)(input)?;

            (input, Blob(bytes.to_owned()))
        }
        v if v > 13 && v % 2 != 0 => {
            let size = (v - 13) / 2;

            let (input, bytes) = take(size)(input)?;

            use TextEncoding::*;
            let bytes = bytes.to_vec();
            let value = match enc {
                UTF8 => String::from_utf8(bytes).unwrap(),
                _ => unimplemented!(),
            };

            (input, Text(value))
        }
        e => {
            return Err(nom::Err::Failure(ParserError(format!(
                "unsupported serial type: {}",
                e
            ))))
        }
    };

    Ok((input, serial_type))
}

/// Decode the B-Tree on the first page
pub fn decode_first_page<'a>(enc: &'a TextEncoding, page: &'a [u8]) -> Result<Btree, BoxError> {
    // first 100 of the first page are for the database header but preserve the
    // original input for the absolute offset seek.
    let input = &page[100..];
    let input = InputContext {
        input,
        original_input: page.to_owned(),
    };
    match decode_btree(enc, input) {
        Ok((_, btree)) => Ok(btree),
        Err(err) => Err(format!("failed to decode: {}", err).into()),
    }
}

/// Decode the B-Tree on a page
pub fn decode<'a>(enc: &'a TextEncoding, input: &'a [u8]) -> Result<Btree, BoxError> {
    let input = InputContext {
        input,
        original_input: input.to_owned(),
    };
    match decode_btree(enc, input) {
        Ok((_, btree)) => Ok(btree),
        Err(err) => Err(format!("failed to decode: {}", err).into()),
    }
}

fn decode_btree<'a>(
    enc: &TextEncoding,
    input: InputContext<'a>,
) -> IResult<InputContext<'a>, Btree> {
    let (input, header) = decode_header(input)?;
    let (input, cell_pointers) = decode_cell_pointers(&header, input)?;
    let (input, cells) = {
        let mut cells = Vec::with_capacity(cell_pointers.len());

        let prev_input = input.clone();

        for cell_pointer in cell_pointers {
            let input = input.seek_at(cell_pointer as usize);

            let res = decode_cell(enc, &header.page_type, input)?;
            cells.push(res.1);
        }

        (prev_input, cells)
    };

    // FIXME: consume the input ??? the cell are always at the end of the page...

    let btree = Btree { header, cells };
    Ok((input, btree))
}

impl<'a> InputContext<'a> {
    fn seek_at(&'a self, offset: usize) -> InputContext<'a> {
        let input = &self.original_input[offset..];
        Self {
            input,
            original_input: self.original_input.clone(),
        }
    }

    fn read_u32(self) -> IResult<InputContext<'a>, u32> {
        let (input, v) = util::read_u32(&self.input)?;
        Ok((
            Self {
                input,
                original_input: self.original_input,
            },
            v,
        ))
    }

    fn read_u16(self) -> IResult<InputContext<'a>, u16> {
        let (input, v) = util::read_u16(&self.input)?;
        Ok((
            Self {
                input,
                original_input: self.original_input,
            },
            v,
        ))
    }

    fn read_u8(self) -> IResult<InputContext<'a>, u8> {
        let (input, v) = util::read_u8(&self.input)?;
        Ok((
            Self {
                input,
                original_input: self.original_input,
            },
            v,
        ))
    }

    fn read_varint(self) -> IResult<InputContext<'a>, u64> {
        let (input, (v, _)) = read_varint(&self.input)?;

        Ok((
            Self {
                input,
                original_input: self.original_input,
            },
            v,
        ))
    }

    fn read_bytes(self, n: usize) -> IResult<InputContext<'a>, &'a [u8]> {
        let (input, bytes) = take(n)(self.input)?;

        Ok((
            Self {
                input,
                original_input: self.original_input,
            },
            bytes,
        ))
    }
}

/// Returns (value, variable size)
fn read_varint<'a>(input: &'a [u8]) -> IResult<&'a [u8], (u64, usize)> {
    let mut v = 0u64;
    let mut i = 0usize;

    loop {
        if i >= 8 {
            break;
        }

        v = (v << 7) + (input[i] & 0x7f) as u64;
        if (input[i] & 0x80) == 0 {
            return Ok((&input[i + 1..], (v, i + 1)));
        }

        i += 1;
    }

    v = (v << 8) + (input[i] & 0xff) as u64;

    let input = &input[9..];
    Ok((input, (v, 9)))
}
