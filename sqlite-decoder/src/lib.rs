pub mod db;
pub mod wal;

pub(crate) fn parse_err<'a>(input: &'a [u8]) -> nom::Err<nom::error::Error<&'a [u8]>> {
    nom::Err::Failure(nom::error::Error {
        input,
        code: nom::error::ErrorKind::Fail,
    })
}
