use crate::parsed::Parsed;
use crate::{byte_reader, ParseError};

pub(crate) fn parse(cursor: usize, bytes: &[u8]) -> Result<Parsed<u16>, ParseError> {
    let (route_tag_first_byte, cursor) = byte_reader::read(cursor, bytes)?;
    let (route_tag_second_byte, cursor) = byte_reader::read(cursor, bytes)?;

    Ok(Parsed::new(
        ((route_tag_first_byte as u16) << 8) + route_tag_second_byte as u16,
        cursor,
    ))
}
