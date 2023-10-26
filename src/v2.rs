use crate::{address_family, ipv4, metric, route_tag, PacketParsable, ParseError};
use std::net::Ipv4Addr;

#[derive(PartialEq, Debug)]
pub struct Entry {
    address_family_identifier: address_family::Identifier,
    route_tag: u16,
    ip_address: Ipv4Addr,
    subnet_mask: Ipv4Addr,
    next_hop: Ipv4Addr,
    metric: u32,
}

pub struct EntriesParser {}

impl EntriesParser {
    fn parse_entry<'a>(
        &'a self,
        mut cursor: usize,
        bytes: &'a [u8],
    ) -> Result<(Entry, usize), ParseError> {
        let parsed = address_family::Identifier::parse(cursor, bytes)?;
        let address_family_identifier = *parsed.get_value();
        cursor = parsed.get_cursor();

        let parsed = route_tag::parse(cursor, bytes)?;
        let route_tag = *parsed.get_value();
        cursor = parsed.get_cursor();

        let parsed = ipv4::parse(cursor, bytes)?;
        let ip_address = *parsed.get_value();
        cursor = parsed.get_cursor();

        let parsed = ipv4::parse(cursor, bytes)?;
        let subnet_mask = *parsed.get_value();
        cursor = parsed.get_cursor();

        let parsed = ipv4::parse(cursor, bytes)?;
        let next_hop = *parsed.get_value();
        cursor = parsed.get_cursor();

        let parsed = metric::parse(cursor, bytes)?;
        let metric = *parsed.get_value();
        cursor = parsed.get_cursor();

        Ok((
            Entry {
                address_family_identifier,
                route_tag,
                ip_address,
                subnet_mask,
                next_hop,
                metric,
            },
            cursor,
        ))
    }
}

impl PacketParsable<Entry> for EntriesParser {
    fn parse_entries(&self, mut cursor: usize, bytes: &[u8]) -> Result<Vec<Entry>, ParseError> {
        let mut entries: Vec<Entry> = vec![];

        if (bytes.len() - 1) <= cursor {
            return Err(ParseError::EmptyRIPEntry(cursor));
        }

        loop {
            if entries.len() >= 25 {
                return Err(ParseError::MaxRIPEntriesNumberExceeded(cursor));
            }

            let res = match self.parse_entry(cursor, bytes) {
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
}
#[cfg(test)]
mod tests {
    use crate::v2::{EntriesParser, Entry};
    use crate::{address_family, PacketParsable};
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_packet_for_single_entry() {
        let parser = EntriesParser {};
        let result = parser.parse_entries(
            4,
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

        let entries = result.unwrap();
        assert_eq!(
            entries,
            vec![Entry {
                address_family_identifier: address_family::Identifier::IP,
                route_tag: 258,
                ip_address: Ipv4Addr::new(192, 0, 2, 100),
                subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
                next_hop: Ipv4Addr::new(192, 0, 2, 111),
                metric: 67305985,
            }]
        );
    }

    #[test]
    fn test_parse_packet_for_multiple_entry() {
        let parser = EntriesParser {};
        let result = parser.parse_entries(
            4,
            vec![
                2, 1, 0, 0, //
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

        let entries = result.unwrap();
        assert_eq!(
            entries,
            vec![
                Entry {
                    address_family_identifier: address_family::Identifier::IP,
                    route_tag: 258,
                    ip_address: Ipv4Addr::new(192, 0, 2, 100),
                    subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
                    next_hop: Ipv4Addr::new(192, 0, 2, 200),
                    metric: 67305985,
                },
                Entry {
                    address_family_identifier: address_family::Identifier::IP,
                    route_tag: 1,
                    ip_address: Ipv4Addr::new(192, 0, 2, 101),
                    subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
                    next_hop: Ipv4Addr::new(192, 0, 2, 201),
                    metric: 1,
                },
                Entry {
                    address_family_identifier: address_family::Identifier::IP,
                    route_tag: 2,
                    ip_address: Ipv4Addr::new(192, 0, 2, 102),
                    subnet_mask: Ipv4Addr::new(255, 255, 255, 0),
                    next_hop: Ipv4Addr::new(192, 0, 2, 202),
                    metric: 2,
                },
            ]
        );
    }
}
