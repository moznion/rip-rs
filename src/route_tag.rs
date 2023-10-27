use crate::parsed::Parsed;
use crate::serializer::SerializeError;
use crate::{byte_reader, parser::ParseError};

pub(crate) fn parse(cursor: usize, bytes: &[u8]) -> Result<Parsed<u16>, ParseError> {
    let (route_tag_first_byte, cursor) = byte_reader::read(cursor, bytes)?;
    let (route_tag_second_byte, cursor) = byte_reader::read(cursor, bytes)?;

    Ok((
        ((route_tag_first_byte as u16) << 8) + route_tag_second_byte as u16,
        cursor,
    ))
}

pub(crate) fn to_bytes(value: u16) -> Result<Vec<u8>, SerializeError> {
    Ok(vec![((value & 0xff00) >> 8) as u8, (value & 0x00ff) as u8])
}
