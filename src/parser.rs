use crate::{command, header, packet, v1, v2, version, zero_bytes};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("insufficient input bytes length; at {0} byte")]
    InsufficientInputBytesLength(usize),
    #[error("unknown command kind {0} has given; at {1} byte")]
    UnknownCommandKind(u8, usize),
    #[error("unknown version {0} has given; at {1} byte")]
    UnknownVersion(u8, usize),
    #[error("version which indicates \"must be discarded\" has given; at {0} byte")]
    MustBeDiscardedVersion(usize),
    #[error("the byte must be zero but was {0} at {1} byte")]
    NotZeroByte(u8, usize),
    #[error("encountered the unknown address family identifier {0}; at {1} byte")]
    UnknownAddressFamilyIdentifier(u16, usize),
    #[error("given packet doesn't have the RIP entry part at; {0} byte")]
    EmptyRIPEntry(usize),
    #[error("the number of RIP entries exceeds the maximum number. it allows to have the entries up to 25 in a packet; at {0} byte")]
    MaxRIPEntriesNumberExceeded(usize),
}

pub enum ParsedPacket {
    V1(packet::Packet<v1::Entry>),
    V2(packet::Packet<v2::Entry>),
}

pub fn parse(bytes: &[u8]) -> Result<ParsedPacket, ParseError> {
    let mut cursor: usize = 0;

    let parsed = command::Kind::parse(cursor, bytes)?;
    let command = *parsed.get_value();
    cursor = parsed.get_cursor();

    let version_byte = match bytes
        .get(cursor)
        .ok_or(ParseError::InsufficientInputBytesLength(cursor))
    {
        Ok(b) => *b,
        Err(e) => {
            return Err(e);
        }
    };
    let version_value = version::Version::from_u8(version_byte);
    cursor += 1;

    cursor = zero_bytes::skip(2, cursor, bytes)?;

    let header = header::Header::new(command, version_value);

    match version_value {
        version::Version::Version1 => match parse_entries(&v1::EntriesParser {}, cursor, bytes) {
            Ok(entries) => Ok(ParsedPacket::V1(packet::Packet::new(header, entries))),
            Err(e) => Err(e),
        },
        version::Version::Version2 => match parse_entries(&v2::EntriesParser {}, cursor, bytes) {
            Ok(entries) => Ok(ParsedPacket::V2(packet::Packet::new(header, entries))),
            Err(e) => Err(e),
        },
        version::Version::MustBeDiscarded => Err(ParseError::MustBeDiscardedVersion(cursor)),
        version::Version::Unknown => Err(ParseError::UnknownCommandKind(version_byte, cursor)),
    }
}

pub(crate) fn parse_entries<T>(
    parser: &dyn PacketParsable<T>,
    mut cursor: usize,
    bytes: &[u8],
) -> Result<Vec<T>, ParseError> {
    let mut entries: Vec<T> = vec![];

    if (bytes.len() - 1) <= cursor {
        return Err(ParseError::EmptyRIPEntry(cursor));
    }

    loop {
        if entries.len() >= 25 {
            return Err(ParseError::MaxRIPEntriesNumberExceeded(cursor));
        }

        let res = match parser.parse_entry(cursor, bytes) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };
        entries.push(res.0);
        cursor = res.1;

        if cursor >= bytes.len() {
            break;
        }
    }

    Ok(entries)
}

pub(crate) trait PacketParsable<T> {
    fn parse_entry<'a>(&'a self, cursor: usize, bytes: &'a [u8]) -> Result<(T, usize), ParseError>;
}

#[cfg(test)]
mod tests {
    use crate::v1::Entry;
    use crate::{address_family, command, header::Header, packet::Packet, parser, version};
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_v1_packet() {
        let result = parser::parse(
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );

        assert_eq!(result.is_ok(), true);

        let packet = match result.unwrap() {
            parser::ParsedPacket::V1(p) => p,
            parser::ParsedPacket::V2(_) => {
                assert_eq!(
                    false, false,
                    "unexpected because given packet is not the v2 packet"
                );
                return;
            }
        };

        let expected_packet = Packet::new(
            Header::new(command::Kind::Response, version::Version::Version1),
            vec![Entry::new(
                address_family::Identifier::IP,
                Ipv4Addr::new(192, 0, 2, 100),
                67305985,
            )],
        );
        assert_eq!(packet, expected_packet);
    }
}