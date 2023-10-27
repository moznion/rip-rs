use crate::packet::Packet;
use crate::v1;
use crate::v2;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SerializeError {
    #[error("encountered the unknown command kind")]
    UnknownCommandKind,
    #[error("encountered the unknown version")]
    UnknownVersion,
    #[error("encountered the unknown address family identifier")]
    UnknownAddressFamilyIdentifier,
}

pub(crate) trait Serializable {
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError>;
}

pub fn serialize_v1_packet(packet: Packet<v1::Entry>) -> Result<Vec<u8>, SerializeError> {
    packet.to_bytes()
}

pub fn serialize_v2_packet(packet: Packet<v2::Entry>) -> Result<Vec<u8>, SerializeError> {
    packet.to_bytes()
}

#[cfg(test)]
mod tests {
    use crate::header::Header;
    use crate::packet::Packet;
    use crate::serializer::{serialize_v1_packet, serialize_v2_packet};
    use crate::{address_family, command, v1, v2, version};
    use std::net::Ipv4Addr;

    #[test]
    fn test_v1_packet_has_single_entry_to_bytes() {
        let packet = Packet::make_v1_packet(
            Header::new(command::Kind::Response, version::Version::Version1),
            vec![v1::Entry::new(
                address_family::Identifier::IP,
                Ipv4Addr::new(192, 0, 2, 100),
                67305985,
            )],
        )
        .unwrap();

        let serialization_result = serialize_v1_packet(packet);

        assert_eq!(serialization_result.is_ok(), true);
        assert_eq!(
            serialization_result.unwrap(),
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
        );
    }

    #[test]
    fn test_v1_packet_has_multi_entries_to_bytes() {
        let packet = Packet::make_v1_packet(
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

        let serialization_result = serialize_v1_packet(packet);

        assert_eq!(serialization_result.is_ok(), true);
        assert_eq!(
            serialization_result.unwrap(),
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
        );
    }

    #[test]
    fn test_v2_packet_has_single_entry_to_bytes() {
        let packet = Packet::make_v2_packet(
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

        let serialization_result = serialize_v2_packet(packet);

        assert_eq!(serialization_result.is_ok(), true);
        assert_eq!(
            serialization_result.unwrap(),
            vec![
                2, 2, 0, 0, //
                0, 2, 1, 2, //
                192, 0, 2, 100, //
                255, 255, 255, 0, //
                192, 0, 2, 111, //
                4, 3, 2, 1, //
            ]
        );
    }

    #[test]
    fn test_v2_packet_has_multi_entries_to_bytes() {
        let packet = Packet::make_v2_packet(
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

        let serialization_result = serialize_v2_packet(packet);

        assert_eq!(serialization_result.is_ok(), true);
        assert_eq!(
            serialization_result.unwrap(),
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
        );
    }
}
