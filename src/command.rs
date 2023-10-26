use crate::parsed::Parsed;
use crate::{byte_reader, ParseError};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Kind {
    Invalid,
    Request,                  // RFC1058
    Response,                 // RFC1058
    TraceOn,                  // RFC1058
    TraceOff,                 // RFC1058
    Reserved,                 // RFC1058
    TriggeredRequest,         // RFC1582
    TriggeredResponse,        // RFC1582
    TriggeredAcknowledgement, // RFC1582
    UpdateRequest,            // RFC2091
    UpdateResponse,           // RFC2091
    UpdateAcknowledge,        // RFC2091
    Unknown,
}

impl Kind {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Kind::Invalid,
            1 => Kind::Request,
            2 => Kind::Response,
            3 => Kind::TraceOn,
            4 => Kind::TraceOff,
            5 => Kind::Reserved,
            6 => Kind::TriggeredRequest,
            7 => Kind::TriggeredResponse,
            8 => Kind::TriggeredAcknowledgement,
            9 => Kind::UpdateRequest,
            10 => Kind::UpdateResponse,
            11 => Kind::UpdateAcknowledge,
            _ => Kind::Unknown,
        }
    }

    pub fn to_u8(&self) -> Option<u8> {
        match self {
            Kind::Invalid => Some(0),
            Kind::Request => Some(1),
            Kind::Response => Some(2),
            Kind::TraceOn => Some(3),
            Kind::TraceOff => Some(4),
            Kind::Reserved => Some(5),
            Kind::TriggeredRequest => Some(6),
            Kind::TriggeredResponse => Some(7),
            Kind::TriggeredAcknowledgement => Some(8),
            Kind::UpdateRequest => Some(9),
            Kind::UpdateResponse => Some(10),
            Kind::UpdateAcknowledge => Some(11),
            Kind::Unknown => None,
        }
    }

    pub(crate) fn parse(cursor: usize, bytes: &[u8]) -> Result<Parsed<Kind>, ParseError> {
        let (command_byte, cursor) = byte_reader::read(cursor, bytes)?;

        let command = match Kind::from_u8(command_byte) {
            Kind::Unknown => {
                return Err(ParseError::UnknownCommandKind(command_byte, cursor));
            }
            _command => _command,
        };

        Ok(Parsed::new(command, cursor))
    }
}
