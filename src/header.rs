use crate::parsed::Parsed;
use crate::parser::ParseError;
use crate::serializer::{Serializable, SerializeError};
use crate::{byte_reader, command, version, zero_bytes};

#[derive(PartialEq, Debug)]
pub struct Header {
    command: command::Kind,
    version: version::Version,
}

pub fn parse(cursor: usize, bytes: &[u8]) -> Result<Parsed<Header>, ParseError> {
    let (command, cursor) = command::Kind::parse(cursor, bytes)?;
    let (version_byte, cursor) = byte_reader::read(cursor, bytes)?;
    let version_value = version::Version::from_u8(version_byte);
    let cursor = zero_bytes::skip(2, cursor, bytes)?;
    let header = Header::new(command, version_value);

    Ok((header, cursor))
}

impl Header {
    pub fn new(command: command::Kind, version: version::Version) -> Self {
        Header { command, version }
    }

    pub fn get_command(&self) -> command::Kind {
        self.command
    }

    pub fn get_version(&self) -> version::Version {
        self.version
    }
}

impl Serializable for Header {
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        let command_bytes = self.get_command().to_bytes()?;
        let version_bytes = self.get_version().to_bytes()?;
        Ok([command_bytes, version_bytes, vec![0, 0]].concat())
    }
}

#[cfg(test)]
mod tests {
    use crate::header::Header;
    use crate::parser::ParseError;
    use crate::serializer::Serializable;
    use crate::{command, header, version};

    #[test]
    fn test_to_bytes() {
        assert_eq!(
            Header::new(command::Kind::Request, version::Version::Version2)
                .to_bytes()
                .unwrap(),
            vec![0x01, 0x02, 0x00, 0x00]
        );
    }

    #[test]
    fn test_parse() {
        let (header, cursor) = header::parse(0, vec![0x01, 0x02, 0x00, 0x00].as_slice()).unwrap();
        assert_eq!(
            header,
            Header::new(command::Kind::Request, version::Version::Version2)
        );
        assert_eq!(cursor, 4);
    }

    #[test]
    fn test_parse_has_non_zero_byte() {
        let err = header::parse(0, vec![0x01, 0x02, 0x01, 0x00].as_slice()).unwrap_err();
        assert_eq!(err, ParseError::NotZeroByte(0x01, 3));

        let err = header::parse(0, vec![0x01, 0x02, 0x00, 0x02].as_slice()).unwrap_err();
        assert_eq!(err, ParseError::NotZeroByte(0x02, 4));
    }
}
