use crate::{byte_reader, ParseError};

pub(crate) fn skip(
    num_of_zero_bytes: usize,
    mut cursor: usize,
    bytes: &[u8],
) -> Result<usize, ParseError> {
    for _ in 0..num_of_zero_bytes {
        let (byte, new_cursor) = byte_reader::read(cursor, bytes)?;
        if byte != 0 {
            return Err(ParseError::NotZeroByte(byte, new_cursor));
        }
        cursor = new_cursor;
    }
    Ok(cursor)
}
