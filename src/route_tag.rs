use crate::parser::Parsed;
use crate::serializer::SerializeError;
use crate::{byte_reader, parser::ParseError};

pub type RouteTag = u16;
pub(crate) fn parse(cursor: usize, bytes: &[u8]) -> Result<Parsed<RouteTag>, ParseError> {
    let (route_tag_first_byte, cursor) = byte_reader::read(cursor, bytes)?;
    let (route_tag_second_byte, cursor) = byte_reader::read(cursor, bytes)?;

    Ok((
        ((route_tag_first_byte as RouteTag) << 8) + route_tag_second_byte as RouteTag,
        cursor,
    ))
}

pub(crate) fn to_bytes(value: RouteTag) -> Result<Vec<u8>, SerializeError> {
    Ok(vec![((value & 0xff00) >> 8) as u8, (value & 0x00ff) as u8])
}
