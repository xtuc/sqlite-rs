pub mod db;
mod util;
pub mod wal;

#[derive(Debug)]
pub(crate) struct ParserError(String);

impl<I> nom::error::ParseError<I> for ParserError {
    fn from_error_kind(_input: I, kind: nom::error::ErrorKind) -> Self {
        ParserError(format!("error {:?}", kind))
    }
    fn append(_input: I, _kind: nom::error::ErrorKind, _other: Self) -> Self {
        todo!()
    }
    fn from_char(_input: I, _: char) -> Self {
        todo!()
    }
    fn or(self, _other: Self) -> Self {
        todo!()
    }
}

pub(crate) type IResult<I, O, E = ParserError> = Result<(I, O), nom::Err<E>>;
