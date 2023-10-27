use crate::byte_reader;
use crate::parser::ParseError;
use crate::parser::Parsed;
use crate::serializer::{Serializable, SerializeError};
use SerializeError::UnknownAddressFamilyIdentifier;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Identifier {
    Unspecified,           // RFC1058
    IP,                    // RFC1058
    AuthenticationPresent, // RFC1388
    Unknown,
}

impl Identifier {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => Identifier::Unspecified,
            2 => Identifier::IP,
            65535 => Identifier::AuthenticationPresent,
            _ => Identifier::Unknown,
        }
    }

    pub fn to_u16(&self) -> Option<u16> {
        match self {
            Identifier::Unspecified => Some(0),
            Identifier::IP => Some(2),
            Identifier::AuthenticationPresent => Some(65535),
            Identifier::Unknown => None,
        }
    }

    pub(crate) fn parse(cursor: usize, bytes: &[u8]) -> Result<Parsed<Identifier>, ParseError> {
        let (address_family_identifier_first_byte, cursor) = byte_reader::read(cursor, bytes)?;
        let (address_family_identifier_second_byte, cursor) = byte_reader::read(cursor, bytes)?;

        let address_family_identifier_value = ((address_family_identifier_first_byte as u16) << 8)
            + address_family_identifier_second_byte as u16;
        let address_family_identifier = match Identifier::from_u16(address_family_identifier_value)
        {
            Identifier::Unknown => {
                return Err(ParseError::UnknownAddressFamilyIdentifier(
                    address_family_identifier_value,
                    cursor - 1,
                ))
            }
            _identifier => _identifier,
        };

        Ok((address_family_identifier, cursor))
    }
}

impl Serializable for Identifier {
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        let v = match self.to_u16() {
            Some(v) => v,
            None => {
                return Err(UnknownAddressFamilyIdentifier);
            }
        };

        Ok(vec![((v & 0xff00) >> 8) as u8, (v & 0x00ff) as u8])
    }
}
#[cfg(test)]
mod tests {
    use crate::address_family::Identifier;
    use crate::parser::ParseError;
    use crate::serializer::{Serializable, SerializeError};

    #[test]
    fn test_parse() {
        let (identifier, cursor) = Identifier::parse(0, vec![0x00, 0x00].as_slice()).unwrap();
        assert_eq!(identifier, Identifier::Unspecified);
        assert_eq!(cursor, 2);
        let (identifier, cursor) = Identifier::parse(0, vec![0x00, 0x02].as_slice()).unwrap();
        assert_eq!(identifier, Identifier::IP);
        assert_eq!(cursor, 2);
        let (identifier, cursor) = Identifier::parse(0, vec![0xff, 0xff].as_slice()).unwrap();
        assert_eq!(identifier, Identifier::AuthenticationPresent);
        assert_eq!(cursor, 2);

        let result = Identifier::parse(0, vec![0x00, 0x01].as_slice());
        assert_eq!(
            result.unwrap_err(),
            ParseError::UnknownAddressFamilyIdentifier(1, 1)
        );
    }

    #[test]
    fn to_bytes() {
        assert_eq!(
            Identifier::Unspecified.to_bytes().unwrap(),
            vec![0x00, 0x00]
        );
        assert_eq!(Identifier::IP.to_bytes().unwrap(), vec![0x00, 0x02]);
        assert_eq!(
            Identifier::AuthenticationPresent.to_bytes().unwrap(),
            vec![0xff, 0xff]
        );
        assert_eq!(
            Identifier::Unknown.to_bytes().unwrap_err(),
            SerializeError::UnknownAddressFamilyIdentifier
        );
    }
}
