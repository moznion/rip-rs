use crate::parser::Parsed;
use crate::serializer::SerializeError::UnknownCommandKind;
use crate::serializer::{Serializable, SerializeError};
use crate::{byte_reader, parser::ParseError};

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

        Ok((command, cursor))
    }
}

impl Serializable for Kind {
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        match self.to_u8() {
            Some(byte) => Ok(vec![byte]),
            None => Err(UnknownCommandKind),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::Kind;
    use crate::parser::ParseError;
    use crate::serializer::{Serializable, SerializeError};

    #[test]
    fn test_parse() {
        let (kind, cursor) = Kind::parse(0, vec![0x00].as_slice()).unwrap();
        assert_eq!(kind, Kind::Invalid);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x01].as_slice()).unwrap();
        assert_eq!(kind, Kind::Request);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x02].as_slice()).unwrap();
        assert_eq!(kind, Kind::Response);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x03].as_slice()).unwrap();
        assert_eq!(kind, Kind::TraceOn);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x04].as_slice()).unwrap();
        assert_eq!(kind, Kind::TraceOff);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x05].as_slice()).unwrap();
        assert_eq!(kind, Kind::Reserved);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x06].as_slice()).unwrap();
        assert_eq!(kind, Kind::TriggeredRequest);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x07].as_slice()).unwrap();
        assert_eq!(kind, Kind::TriggeredResponse);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x08].as_slice()).unwrap();
        assert_eq!(kind, Kind::TriggeredAcknowledgement);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x09].as_slice()).unwrap();
        assert_eq!(kind, Kind::UpdateRequest);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x0a].as_slice()).unwrap();
        assert_eq!(kind, Kind::UpdateResponse);
        assert_eq!(cursor, 1);
        let (kind, cursor) = Kind::parse(0, vec![0x0b].as_slice()).unwrap();
        assert_eq!(kind, Kind::UpdateAcknowledge);
        assert_eq!(cursor, 1);

        assert_eq!(
            Kind::parse(0, vec![0xff].as_slice()).unwrap_err(),
            ParseError::UnknownCommandKind(0xff, 1)
        );
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(Kind::Invalid.to_bytes().unwrap(), vec![0]);
        assert_eq!(Kind::Request.to_bytes().unwrap(), vec![1]);
        assert_eq!(Kind::Response.to_bytes().unwrap(), vec![2]);
        assert_eq!(Kind::TraceOn.to_bytes().unwrap(), vec![3]);
        assert_eq!(Kind::TraceOff.to_bytes().unwrap(), vec![4]);
        assert_eq!(Kind::Reserved.to_bytes().unwrap(), vec![5]);
        assert_eq!(Kind::TriggeredRequest.to_bytes().unwrap(), vec![6]);
        assert_eq!(Kind::TriggeredResponse.to_bytes().unwrap(), vec![7]);
        assert_eq!(Kind::TriggeredAcknowledgement.to_bytes().unwrap(), vec![8]);
        assert_eq!(Kind::UpdateRequest.to_bytes().unwrap(), vec![9]);
        assert_eq!(Kind::UpdateResponse.to_bytes().unwrap(), vec![10]);
        assert_eq!(Kind::UpdateAcknowledge.to_bytes().unwrap(), vec![11]);

        assert_eq!(
            Kind::Unknown.to_bytes().unwrap_err(),
            SerializeError::UnknownCommandKind
        );
    }
}
