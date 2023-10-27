use crate::packet::PacketError;
use crate::parser::ParseError::InvalidPacket;
use crate::{header, packet, v1, v2, version};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("insufficient input bytes length; at {0} byte")]
    InsufficientInputBytesLength(usize),
    #[error("unknown command kind {0} has given; at {1} byte")]
    UnknownCommandKind(u8, usize),
    #[error("unknown version has given; at {0} byte")]
    UnknownVersion(usize),
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
    #[error("invalid packet: {0}")]
    InvalidPacket(PacketError),
}

#[derive(Debug)]
pub enum ParsedPacket {
    V1(packet::Packet<v1::Entry>),
    V2(packet::Packet<v2::Entry>),
}

/// Parsed is a tuple type which has a T-typed value end a cursor for bytes reading.
pub type Parsed<T> = (T, usize);

pub fn parse(bytes: &[u8]) -> Result<ParsedPacket, ParseError> {
    let (header, cursor) = header::parse(0, bytes)?;

    match header.get_version() {
        version::Version::Version1 => match parse_entries(&v1::EntriesParser {}, cursor, bytes) {
            Ok(entries) => Ok(ParsedPacket::V1(
                packet::Packet::make_v1_packet(header, entries).unwrap(),
            )),
            Err(e) => Err(e),
        },
        version::Version::Version2 => match parse_entries(&v2::EntriesParser {}, cursor, bytes) {
            Ok(entries) => Ok(ParsedPacket::V2(
                packet::Packet::make_v2_packet(header, entries).unwrap(),
            )),
            Err(e) => Err(e),
        },
        version::Version::MustBeDiscarded => Err(ParseError::MustBeDiscardedVersion(2)),
        version::Version::Unknown => Err(ParseError::UnknownVersion(2)),
    }
}

pub fn parse_v1(bytes: &[u8]) -> Result<packet::Packet<v1::Entry>, ParseError> {
    let (header, cursor) = header::parse(0, bytes)?;

    match parse_entries(&v1::EntriesParser {}, cursor, bytes) {
        Ok(entries) => match packet::Packet::make_v1_packet(header, entries) {
            Ok(p) => Ok(p),
            Err(e) => Err(InvalidPacket(e)),
        },
        Err(e) => Err(e),
    }
}

pub fn parse_v2(bytes: &[u8]) -> Result<packet::Packet<v2::Entry>, ParseError> {
    let (header, cursor) = header::parse(0, bytes)?;

    match parse_entries(&v2::EntriesParser {}, cursor, bytes) {
        Ok(entries) => match packet::Packet::make_v2_packet(header, entries) {
            Ok(p) => Ok(p),
            Err(e) => Err(InvalidPacket(e)),
        },
        Err(e) => Err(e),
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
    use crate::packet::PacketError::VersionInHeaderConflicted;
    use crate::parser::ParseError;
    use crate::parser::ParseError::{InsufficientInputBytesLength, InvalidPacket};
    use crate::{address_family, command, header::Header, packet::Packet, parser, v1, v2, version};
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_v1_packet_for_single_entry() {
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
        let expected_packet = Packet::make_v1_packet(
            Header::new(command::Kind::Response, version::Version::Version1),
            vec![v1::Entry::new(
                address_family::Identifier::IP,
                Ipv4Addr::new(192, 0, 2, 100),
                67305985,
            )],
        )
        .unwrap();
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn test_parse_v1_packet_for_multiple_entry() {
        let result = parser::parse(
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                4, 3, 2, 1, //
                0, 2, 0, 0, //
                192, 0, 2, 101, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                0, 0, 0, 1, //
                0, 2, 0, 0, //
                192, 0, 2, 102, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                0, 0, 0, 2, //
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
        let expected_packet = Packet::make_v1_packet(
            Header::new(command::Kind::Response, version::Version::Version1),
            vec![
                v1::Entry::new(
                    address_family::Identifier::IP,
                    Ipv4Addr::new(192, 0, 2, 100),
                    67305985,
                ),
                v1::Entry::new(
                    address_family::Identifier::IP,
                    Ipv4Addr::new(192, 0, 2, 101),
                    1,
                ),
                v1::Entry::new(
                    address_family::Identifier::IP,
                    Ipv4Addr::new(192, 0, 2, 102),
                    2,
                ),
            ],
        )
        .unwrap();
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn test_parse_v2_packet_for_single_entry() {
        let result = parser::parse(
            vec![
                2, 2, 0, 0, //
                0, 2, 1, 2, //
                192, 0, 2, 100, //
                255, 255, 255, 0, //
                192, 0, 2, 111, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );

        assert_eq!(result.is_ok(), true);

        let packet = match result.unwrap() {
            parser::ParsedPacket::V1(_) => {
                assert_eq!(
                    false, false,
                    "unexpected because given packet is not the v1 packet"
                );
                return;
            }
            parser::ParsedPacket::V2(p) => p,
        };
        let expected_packet = Packet::make_v2_packet(
            Header::new(command::Kind::Response, version::Version::Version2),
            vec![v2::Entry::new(
                address_family::Identifier::IP,
                258,
                Ipv4Addr::new(192, 0, 2, 100),
                Ipv4Addr::new(255, 255, 255, 0),
                Ipv4Addr::new(192, 0, 2, 111),
                67305985,
            )],
        )
        .unwrap();
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn test_parse_v2_packet_for_multiple_entry() {
        let result = parser::parse(
            vec![
                2, 2, 0, 0, //
                0, 2, 1, 2, //
                192, 0, 2, 100, //
                255, 255, 255, 0, //
                192, 0, 2, 200, //
                4, 3, 2, 1, //
                0, 2, 0, 1, //
                192, 0, 2, 101, //
                255, 255, 255, 0, //
                192, 0, 2, 201, //
                0, 0, 0, 1, //
                0, 2, 0, 2, //
                192, 0, 2, 102, //
                255, 255, 255, 0, //
                192, 0, 2, 202, //
                0, 0, 0, 2, //
            ]
            .as_slice(),
        );
        assert_eq!(result.is_ok(), true);

        let packet = match result.unwrap() {
            parser::ParsedPacket::V1(_) => {
                assert_eq!(
                    false, false,
                    "unexpected because given packet is not the v1 packet"
                );
                return;
            }
            parser::ParsedPacket::V2(p) => p,
        };
        let expected_packet = Packet::make_v2_packet(
            Header::new(command::Kind::Response, version::Version::Version2),
            vec![
                v2::Entry::new(
                    address_family::Identifier::IP,
                    258,
                    Ipv4Addr::new(192, 0, 2, 100),
                    Ipv4Addr::new(255, 255, 255, 0),
                    Ipv4Addr::new(192, 0, 2, 200),
                    67305985,
                ),
                v2::Entry::new(
                    address_family::Identifier::IP,
                    1,
                    Ipv4Addr::new(192, 0, 2, 101),
                    Ipv4Addr::new(255, 255, 255, 0),
                    Ipv4Addr::new(192, 0, 2, 201),
                    1,
                ),
                v2::Entry::new(
                    address_family::Identifier::IP,
                    2,
                    Ipv4Addr::new(192, 0, 2, 102),
                    Ipv4Addr::new(255, 255, 255, 0),
                    Ipv4Addr::new(192, 0, 2, 202),
                    2,
                ),
            ],
        )
        .unwrap();
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn test_parse_insufficient_length_bytes_for_v1() {
        let result = parser::parse(
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                4, 3, 2, // missing a trailing byte
            ]
            .as_slice(),
        );

        assert_eq!(
            result.unwrap_err(),
            ParseError::InsufficientInputBytesLength(23)
        );
    }

    #[test]
    fn test_parse_insufficient_length_bytes_for_v2() {
        let result = parser::parse(
            vec![
                2, 2, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                255, 255, 255, 0, //
                0, 0, 0, 0, //
                4, 3, 2, // missing a trailing byte
            ]
            .as_slice(),
        );

        assert_eq!(
            result.unwrap_err(),
            ParseError::InsufficientInputBytesLength(23)
        );
    }

    #[test]
    fn test_parse_bytes_must_be_discarded() {
        let result = parser::parse(
            vec![
                2, 0, 0, 0, // version byte is 0 (must be discarded)
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                255, 255, 255, 0, //
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );

        assert_eq!(result.unwrap_err(), ParseError::MustBeDiscardedVersion(2),);
    }

    #[test]
    fn test_parse_bytes_of_unknown_version() {
        let result = parser::parse(
            vec![
                2, 255, 0, 0, // version byte is 255 (unknown)
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                255, 255, 255, 0, //
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );

        assert_eq!(result.unwrap_err(), ParseError::UnknownVersion(2));
    }

    #[test]
    fn test_parse_empty_entry_part() {
        let result = parser::parse(vec![2, 2, 0, 0].as_slice());

        assert_eq!(result.unwrap_err(), ParseError::EmptyRIPEntry(4));
    }

    #[test]
    fn test_parse_bytes_which_has_the_number_of_entries_that_exceeds_max_limit() {
        let result = parser::parse(
            vec![
                2, 2, 0, 0, //
                0, 2, 1, 2, 192, 0, 2, 101, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 1, //
                0, 2, 1, 2, 192, 0, 2, 102, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 2, //
                0, 2, 1, 2, 192, 0, 2, 103, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 3, //
                0, 2, 1, 2, 192, 0, 2, 104, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 4, //
                0, 2, 1, 2, 192, 0, 2, 105, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 5, //
                0, 2, 1, 2, 192, 0, 2, 106, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 6, //
                0, 2, 1, 2, 192, 0, 2, 107, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 7, //
                0, 2, 1, 2, 192, 0, 2, 108, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 8, //
                0, 2, 1, 2, 192, 0, 2, 109, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 9, //
                0, 2, 1, 2, 192, 0, 2, 110, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 10, //
                0, 2, 1, 2, 192, 0, 2, 111, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 11, //
                0, 2, 1, 2, 192, 0, 2, 112, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 12, //
                0, 2, 1, 2, 192, 0, 2, 113, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 13, //
                0, 2, 1, 2, 192, 0, 2, 114, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 14, //
                0, 2, 1, 2, 192, 0, 2, 115, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 15, //
                0, 2, 1, 2, 192, 0, 2, 116, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 16, //
                0, 2, 1, 2, 192, 0, 2, 117, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 17, //
                0, 2, 1, 2, 192, 0, 2, 118, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 18, //
                0, 2, 1, 2, 192, 0, 2, 119, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 19, //
                0, 2, 1, 2, 192, 0, 2, 120, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 20, //
                0, 2, 1, 2, 192, 0, 2, 121, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 21, //
                0, 2, 1, 2, 192, 0, 2, 122, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 22, //
                0, 2, 1, 2, 192, 0, 2, 123, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 23, //
                0, 2, 1, 2, 192, 0, 2, 124, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 24, //
                0, 2, 1, 2, 192, 0, 2, 125, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 25, //
                0, 2, 1, 2, 192, 0, 2, 126, 255, 255, 255, 0, 192, 0, 2, 200, 0, 0, 0, 26, //
            ]
            .as_slice(),
        );

        assert_eq!(
            result.unwrap_err(),
            ParseError::MaxRIPEntriesNumberExceeded(504)
        );
    }

    #[test]
    fn test_parse_v1() {
        let result = parser::parse_v1(
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

        let expected_packet = Packet::make_v1_packet(
            Header::new(command::Kind::Response, version::Version::Version1),
            vec![v1::Entry::new(
                address_family::Identifier::IP,
                Ipv4Addr::new(192, 0, 2, 100),
                67305985,
            )],
        )
        .unwrap();
        assert_eq!(result.unwrap(), expected_packet);
    }

    #[test]
    fn test_parse_v1_with_conflict_version() {
        let result = parser::parse_v1(
            vec![
                2, 2, 0, 0, // the second byte is 2 (i.e version 2)
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(
            result.unwrap_err(),
            InvalidPacket(VersionInHeaderConflicted)
        );
    }

    #[test]
    fn test_parse_v1_with_insufficient_bytes() {
        let result = parser::parse_v1(
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                4, 3, 2, // trailing byte is missing
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), InsufficientInputBytesLength(23));
    }

    #[test]
    fn test_parse_v2() {
        let result = parser::parse_v2(
            vec![
                2, 2, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                255, 255, 255, 0, //
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );

        let expected_packet = Packet::make_v2_packet(
            Header::new(command::Kind::Response, version::Version::Version2),
            vec![v2::Entry::new(
                address_family::Identifier::IP,
                0,
                Ipv4Addr::new(192, 0, 2, 100),
                Ipv4Addr::new(255, 255, 255, 0),
                Ipv4Addr::new(0, 0, 0, 0),
                67305985,
            )],
        )
        .unwrap();
        assert_eq!(result.unwrap(), expected_packet);
    }

    #[test]
    fn test_parse_v2_with_conflict_version() {
        let result = parser::parse_v2(
            vec![
                2, 1, 0, 0, // the second byte is 1 (i.e version 1)
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                255, 255, 255, 0, //
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(
            result.unwrap_err(),
            InvalidPacket(VersionInHeaderConflicted)
        );
    }

    #[test]
    fn test_parse_v2_with_insufficient_bytes() {
        let result = parser::parse_v2(
            vec![
                2, 2, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                4, 3, 2, // trailing byte is missing
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), InsufficientInputBytesLength(23));
    }
}
