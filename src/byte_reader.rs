use crate::ParseError;

pub(crate) fn read(mut cursor: usize, bytes: &[u8]) -> Result<(u8, usize), ParseError> {
    let b = match bytes
        .get(cursor)
        .ok_or(ParseError::InsufficientInputBytesLength(cursor))
    {
        Ok(b) => *b,
        Err(e) => {
            return Err(e);
        }
    };
    cursor += 1;

    Ok((b, cursor))
}
