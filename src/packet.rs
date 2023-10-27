use crate::packet::PacketError::VersionInHeaderConflicted;
use crate::serializer::{Serializable, SerializeError};
use crate::{header, v1, v2, version};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PacketError {
    #[error("version in the header conflicted")]
    VersionInHeaderConflicted,
    #[error("the number of RIP entries exceeds the maximum number. it allows to have the entries up to 25 in a packet")]
    MaxRIPEntriesNumberExceeded,
}

#[derive(PartialEq, Debug)]
pub struct Packet<T> {
    header: header::Header,
    entries: Vec<T>,
}

impl<T> Packet<T> {
    fn new(header: header::Header, entries: Vec<T>) -> Result<Self, PacketError> {
        if entries.len() > 25 {
            return Err(PacketError::MaxRIPEntriesNumberExceeded);
        }

        Ok(Packet { header, entries })
    }

    pub fn get_header(&self) -> &header::Header {
        &self.header
    }

    pub fn get_entries(&self) -> &Vec<T> {
        &self.entries
    }
}

impl Packet<v1::Entry> {
    pub fn make_v1_packet(
        header: header::Header,
        entries: Vec<v1::Entry>,
    ) -> Result<Self, PacketError> {
        let ver = header.get_version();
        if ver != version::Version::Version1 {
            return Err(VersionInHeaderConflicted);
        }
        Packet::new(header, entries)
    }
}

impl Packet<v2::Entry> {
    pub fn make_v2_packet(
        header: header::Header,
        entries: Vec<v2::Entry>,
    ) -> Result<Self, PacketError> {
        let ver = header.get_version();
        if ver != version::Version::Version2 {
            return Err(VersionInHeaderConflicted);
        }
        Packet::new(header, entries)
    }
}

impl<T: Serializable> Serializable for Packet<T> {
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        let mut entries_bytes = vec![];

        for entry in self.get_entries() {
            entries_bytes.extend(entry.to_bytes()?);
        }

        Ok([self.get_header().to_bytes()?, entries_bytes].concat())
    }
}

#[cfg(test)]
mod tests {
    use crate::address_family::Identifier;
    use crate::header::Header;
    use crate::packet::{Packet, PacketError};
    use crate::serializer::Serializable;
    use crate::{command, v1, version};
    use std::net::Ipv4Addr;

    #[test]
    fn test_make_v1_packet_on_version_conflict() {
        assert_eq!(
            Packet::make_v1_packet(
                Header::new(command::Kind::Response, version::Version::Version2),
                vec![]
            )
            .unwrap_err(),
            PacketError::VersionInHeaderConflicted
        );
    }

    #[test]
    fn test_make_v2_packet_on_version_conflict() {
        assert_eq!(
            Packet::make_v2_packet(
                Header::new(command::Kind::Response, version::Version::Version1),
                vec![]
            )
            .unwrap_err(),
            PacketError::VersionInHeaderConflicted
        );
    }

    #[test]
    fn test_max_entries_num_exceeded() {
        let result = Packet::new(
            Header::new(command::Kind::Response, version::Version::Version1),
            vec![
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 101), 1),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 102), 2),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 103), 3),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 104), 4),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 105), 5),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 106), 6),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 107), 7),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 108), 8),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 109), 9),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 110), 10),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 111), 11),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 112), 12),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 113), 13),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 114), 14),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 115), 15),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 116), 16),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 117), 17),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 118), 18),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 119), 19),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 120), 20),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 121), 21),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 122), 22),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 123), 23),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 124), 24),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 125), 25),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 126), 26),
            ],
        );
        assert_eq!(
            result.unwrap_err(),
            PacketError::MaxRIPEntriesNumberExceeded
        )
    }

    #[test]
    fn test_parse_single_packet() {
        let packet = Packet::new(
            Header::new(command::Kind::Response, version::Version::Version1),
            vec![v1::Entry::new(
                Identifier::IP,
                Ipv4Addr::new(192, 0, 2, 101),
                1,
            )],
        )
        .unwrap();

        assert_eq!(
            packet.to_bytes().unwrap(),
            vec![
                0x02, 0x01, 0x00, 0x00, //
                0x00, 0x02, 0x00, 0x00, //
                0xc0, 0x00, 0x02, 0x65, //
                0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x01, //
            ]
        );
    }

    #[test]
    fn test_parse_multi_packets() {
        let packet = Packet::new(
            Header::new(command::Kind::Response, version::Version::Version1),
            vec![
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 101), 1),
                v1::Entry::new(Identifier::IP, Ipv4Addr::new(192, 0, 2, 102), 2),
            ],
        )
        .unwrap();

        assert_eq!(
            packet.to_bytes().unwrap(),
            vec![
                0x02, 0x01, 0x00, 0x00, //
                0x00, 0x02, 0x00, 0x00, //
                0xc0, 0x00, 0x02, 0x65, //
                0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x01, //
                0x00, 0x02, 0x00, 0x00, //
                0xc0, 0x00, 0x02, 0x66, //
                0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x00, //
                0x00, 0x00, 0x00, 0x02, //
            ]
        );
    }
}
