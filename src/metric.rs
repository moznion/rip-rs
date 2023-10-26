use crate::parsed::Parsed;
use crate::{byte_reader, ParseError};

pub(crate) fn parse(cursor: usize, bytes: &[u8]) -> Result<Parsed<u32>, ParseError> {
    let (metric_first_byte, cursor) = byte_reader::read(cursor, bytes)?;
    let (metric_second_byte, cursor) = byte_reader::read(cursor, bytes)?;
    let (metric_third_byte, cursor) = byte_reader::read(cursor, bytes)?;
    let (metric_fourth_byte, cursor) = byte_reader::read(cursor, bytes)?;

    Ok(Parsed::new(
        ((metric_first_byte as u32) << 24)
            + ((metric_second_byte as u32) << 16)
            + ((metric_third_byte as u32) << 8)
            + metric_fourth_byte as u32,
        cursor,
    ))
}
