pub mod address_family;
mod byte_reader;
pub mod command;
mod ipv4;
mod metric;
mod parsed;
mod route_tag;
pub mod v1;
pub mod v2;
pub mod version;
mod zero_bytes;

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

pub enum PacketEnvelope {
    V1(Packet<v1::Entry>),
    V2(Packet<v2::Entry>),
}

#[derive(PartialEq, Debug)]
pub struct Header {
    command: command::Kind,
    version: version::Version,
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

#[derive(PartialEq, Debug)]
pub struct Packet<T> {
    header: Header,
    entries: Vec<T>,
}

impl<T> Packet<T> {
    pub fn new(header: Header, entries: Vec<T>) -> Self {
        Packet { header, entries }
    }

    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn get_entries(&self) -> &Vec<T> {
        &self.entries
    }
}

pub fn parse(bytes: &[u8]) -> Result<PacketEnvelope, ParseError> {
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

    let header = Header::new(command, version_value);

    match version_value {
        version::Version::Version1 => {
            let parser = v1::EntriesParser {};
            match parser.parse_entries(cursor, bytes) {
                Ok(entries) => Ok(PacketEnvelope::V1(Packet::new(header, entries))),
                Err(e) => Err(e),
            }
        }
        version::Version::Version2 => {
            let parser = v2::EntriesParser {};
            match parser.parse_entries(cursor, bytes) {
                Ok(entries) => Ok(PacketEnvelope::V2(Packet::new(header, entries))),
                Err(e) => Err(e),
            }
        }
        version::Version::MustBeDiscarded => Err(ParseError::MustBeDiscardedVersion(cursor)),
        version::Version::Unknown => Err(ParseError::UnknownCommandKind(version_byte, cursor)),
    }
}

pub(crate) trait PacketParsable<T> {
    fn parse_entries(&self, cursor: usize, bytes: &[u8]) -> Result<Vec<T>, ParseError>;
}

#[cfg(test)]
mod tests {
    use crate::v1::Entry;
    use crate::{address_family, command, version, Header, Packet, PacketEnvelope};
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_v1_packet() {
        let result = crate::parse(
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
            PacketEnvelope::V1(p) => p,
            PacketEnvelope::V2(_) => {
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
