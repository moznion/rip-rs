use crate::{address_family, metric, zero_bytes, PacketParsable, ParseError};
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
    use crate::v1::{EntriesParser, Entry};
    use crate::{address_family, PacketParsable};
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_packet_for_single_entry() {
        let parser = EntriesParser {};
        let result = parser.parse_entries(
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
        let result = parser.parse_entries(
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
