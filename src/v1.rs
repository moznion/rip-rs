use crate::serializer::{Serializable, SerializeError};
use crate::{address_family, ipv4, metric, parser::PacketParsable, parser::ParseError, zero_bytes};
use std::net::Ipv4Addr;

#[derive(PartialEq, Debug)]
pub struct Entry {
    address_family_identifier: address_family::Identifier,
    ip_address: Ipv4Addr,
    metric: u32,
}

impl Entry {
    pub fn new(
        address_family_identifier: address_family::Identifier,
        ip_address: Ipv4Addr,
        metric: u32,
    ) -> Self {
        Entry {
            address_family_identifier,
            ip_address,
            metric,
        }
    }

    pub fn get_address_family_identifier(&self) -> address_family::Identifier {
        self.address_family_identifier
    }

    pub fn get_ip_address(&self) -> Ipv4Addr {
        self.ip_address
    }

    pub fn get_metric(&self) -> u32 {
        self.metric
    }
}

impl Serializable for Entry {
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        Ok([
            self.get_address_family_identifier().to_bytes()?,
            vec![0, 0],
            ipv4::to_bytes(self.get_ip_address())?,
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            metric::to_bytes(self.get_metric())?,
        ]
        .concat())
    }
}

pub struct EntriesParser {}

impl PacketParsable<Entry> for EntriesParser {
    fn parse_entry<'a>(
        &'a self,
        mut cursor: usize,
        bytes: &'a [u8],
    ) -> Result<(Entry, usize), ParseError> {
        let parsed = address_family::Identifier::parse(cursor, bytes)?;
        let address_family_identifier = *parsed.get_value();
        cursor = parsed.get_cursor();

        cursor = zero_bytes::skip(2, cursor, bytes)?;

        let parsed = crate::ipv4::parse(cursor, bytes)?;
        let ip_address = *parsed.get_value();
        cursor = parsed.get_cursor();

        cursor = zero_bytes::skip(8, cursor, bytes)?;

        let parsed_metric = metric::parse(cursor, bytes)?;
        let metric = *parsed_metric.get_value();
        cursor = parsed_metric.get_cursor();

        Ok((
            Entry {
                address_family_identifier,
                ip_address,
                metric,
            },
            cursor,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::v1::{EntriesParser, Entry};
    use crate::{address_family, parser};
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_packet_for_single_entry() {
        let parser = EntriesParser {};
        let result = parser::parse_entries(
            &parser,
            4,
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

        let entries = result.unwrap();
        assert_eq!(
            entries,
            vec![Entry {
                address_family_identifier: address_family::Identifier::IP,
                ip_address: Ipv4Addr::new(192, 0, 2, 100),
                metric: 67305985,
            }]
        );
    }

    #[test]
    fn test_parse_packet_for_multiple_entry() {
        let parser = EntriesParser {};
        let result = parser::parse_entries(
            &parser,
            4,
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

        let entries = result.unwrap();
        assert_eq!(
            entries,
            vec![
                Entry {
                    address_family_identifier: address_family::Identifier::IP,
                    ip_address: Ipv4Addr::new(192, 0, 2, 100),
                    metric: 67305985,
                },
                Entry {
                    address_family_identifier: address_family::Identifier::IP,
                    ip_address: Ipv4Addr::new(192, 0, 2, 101),
                    metric: 1,
                },
                Entry {
                    address_family_identifier: address_family::Identifier::IP,
                    ip_address: Ipv4Addr::new(192, 0, 2, 102),
                    metric: 2,
                },
            ]
        );
    }
}
