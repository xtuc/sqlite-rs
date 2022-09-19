use crate::IResult;
use nom::bytes::complete::take;

pub(crate) fn read_u32(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, value) = take(4usize)(input)?;
    Ok((input, u32::from_be_bytes(value.try_into().unwrap())))
}

pub(crate) fn read_u16(input: &[u8]) -> IResult<&[u8], u16> {
    let (input, value) = take(2usize)(input)?;
    Ok((input, u16::from_be_bytes(value.try_into().unwrap())))
}

pub(crate) fn read_u8(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, value) = take(1usize)(input)?;
    Ok((input, u8::from_be_bytes(value.try_into().unwrap())))
}
