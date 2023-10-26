use crate::byte_reader;
use crate::parsed::Parsed;
use crate::parser::ParseError;

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

        Ok(Parsed::new(address_family_identifier, cursor))
    }
}
