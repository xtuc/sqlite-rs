//! https://sqlite.org/fileformat.html#walformat

use nom::bytes::complete::take;
use nom::IResult;
use sqlite_types::{
    Wal, WalFrame, WalFrameHeader, WalHeader, MAGIC_NUMBER_1, MAGIC_NUMBER_2, SUPPORTED_FILE_FORMAT,
};

type BoxError = Box<dyn std::error::Error>;

pub fn decode(input: &[u8]) -> Result<Wal, BoxError> {
    match decode_wal(input) {
        Ok((_, wal)) => Ok(wal),
        Err(err) => Err(format!("failed to decode: {}", err).into()),
    }
}

fn decode_wal(input: &[u8]) -> IResult<&[u8], Wal> {
    let (input, input_header) = take(32usize)(input)?;
    let (_, header) = decode_header(&input_header)?;

    let mut frames = vec![];
    let mut input = input;
    loop {
        if input.len() < header.page_size as usize {
            // EOF or not enough bytes to continue
            break;
        }

        let ret = decode_frame(&input, &header)?;
        input = ret.0;
        frames.push(ret.1);
    }
    Ok((input, Wal { header, frames }))
}

fn read_u32(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, value) = take(4usize)(input)?;
    Ok((input, u32::from_be_bytes(value.try_into().unwrap())))
}

fn decode_header(input: &[u8]) -> IResult<&[u8], WalHeader> {
    let (input, magic_number) = read_u32(input)?;

    if magic_number != MAGIC_NUMBER_1 && magic_number != MAGIC_NUMBER_2 {
        // FIXME: return error
        panic!("magic number not found, got: {:?}", magic_number);
    }

    let (input, file_format) = read_u32(input)?;

    if file_format != SUPPORTED_FILE_FORMAT {
        // FIXME: return error
        panic!("unsupported file format");
    }

    let (input, page_size) = read_u32(input)?;
    let (input, checkpoint_seq) = read_u32(input)?;
    let (input, salt_1) = read_u32(input)?;
    let (input, salt_2) = read_u32(input)?;
    let (input, checksum_1) = read_u32(input)?;
    let (input, checksum_2) = read_u32(input)?;

    Ok((
        input,
        WalHeader {
            magic_number,
            file_format,
            page_size,
            checkpoint_seq,
            salt_1,
            salt_2,
            checksum_1,
            checksum_2,
        },
    ))
}

fn decode_frame_header(input: &[u8]) -> IResult<&[u8], WalFrameHeader> {
    let (input, page_number) = read_u32(input)?;
    let (input, db_size_after_commit) = read_u32(input)?;
    let (input, salt_1) = read_u32(input)?;
    let (input, salt_2) = read_u32(input)?;
    let (input, checksum_1) = read_u32(input)?;
    let (input, checksum_2) = read_u32(input)?;

    Ok((
        input,
        WalFrameHeader {
            page_number,
            db_size_after_commit,
            salt_1,
            salt_2,
            checksum_1,
            checksum_2,
        },
    ))
}

fn decode_frame<'a, 'b>(input: &'a [u8], wal_header: &'b WalHeader) -> IResult<&'a [u8], WalFrame> {
    let (input, input_frame_header) = take(24usize)(input)?;
    let (_, frame_header) = decode_frame_header(&input_frame_header)?;

    if wal_header.salt_1 != frame_header.salt_1 || wal_header.salt_2 != frame_header.salt_2 {
        // FIXME: return error
        panic!("Salt don't match");
    }

    // FIXME: check for `The checksum values in the final 8 bytes of the frame-header exactly match the checksum computed consecutively on the first 24 bytes of the WAL header and the first 8 bytes and the content of all frames up to and including the current frame.`

    let (input, data) = take(wal_header.page_size)(input)?;

    Ok((
        input,
        WalFrame {
            header: frame_header,
            data: data.to_owned(),
        },
    ))
}
