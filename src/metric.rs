use crate::parsed::Parsed;
use crate::serializer::SerializeError;
use crate::{byte_reader, parser::ParseError};

pub(crate) fn parse(cursor: usize, bytes: &[u8]) -> Result<Parsed<u32>, ParseError> {
    let (metric_first_byte, cursor) = byte_reader::read(cursor, bytes)?;
    let (metric_second_byte, cursor) = byte_reader::read(cursor, bytes)?;
    let (metric_third_byte, cursor) = byte_reader::read(cursor, bytes)?;
    let (metric_fourth_byte, cursor) = byte_reader::read(cursor, bytes)?;

    Ok((
        ((metric_first_byte as u32) << 24)
            + ((metric_second_byte as u32) << 16)
            + ((metric_third_byte as u32) << 8)
            + metric_fourth_byte as u32,
        cursor,
    ))
}

pub(crate) fn to_bytes(value: u32) -> Result<Vec<u8>, SerializeError> {
    Ok(vec![
        ((value & 0xff000000) >> 24) as u8,
        ((value & 0x00ff0000) >> 16) as u8,
        ((value & 0x0000ff00) >> 8) as u8,
        (value & 0x000000ff) as u8,
    ])
}
