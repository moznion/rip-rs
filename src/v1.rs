use crate::metric::Metric;
use crate::serializer::{Serializable, SerializeError};
use crate::{address_family, ipv4, metric, parser::PacketParsable, parser::ParseError, zero_bytes};
use std::net::Ipv4Addr;

#[derive(PartialEq, Debug)]
pub struct Entry {
    address_family_identifier: address_family::Identifier,
    ip_address: Ipv4Addr,
    metric: Metric,
}

impl Entry {
    pub fn new(
        address_family_identifier: address_family::Identifier,
        ip_address: Ipv4Addr,
        metric: Metric,
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

    pub fn get_metric(&self) -> Metric {
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
        cursor: usize,
        bytes: &'a [u8],
    ) -> Result<(Entry, usize), ParseError> {
        let (address_family_identifier, cursor) = address_family::Identifier::parse(cursor, bytes)?;

        let cursor = zero_bytes::skip(2, cursor, bytes)?;

        let (ip_address, cursor) = ipv4::parse(cursor, bytes)?;

        let cursor = zero_bytes::skip(8, cursor, bytes)?;

        let (metric, cursor) = metric::parse(cursor, bytes)?;

        Ok((
            Entry::new(address_family_identifier, ip_address, metric),
            cursor,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ParseError::NotZeroByte;
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

    #[test]
    fn test_parse_packet_which_has_not_zero_byte() {
        let parser = EntriesParser {};
        let result = parser::parse_entries(
            &parser,
            4,
            vec![
                2, 1, 0, 0, //
                0, 2, 1, 0, // the third byte is not zero
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), NotZeroByte(1, 7));

        let result = parser::parse_entries(
            &parser,
            4,
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 1, // the fourth byte is not zero
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), NotZeroByte(1, 8));

        let result = parser::parse_entries(
            &parser,
            4,
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                1, 0, 0, 0, // the first byte is not zero
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), NotZeroByte(1, 13));

        let result = parser::parse_entries(
            &parser,
            4,
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 1, 0, 0, // the second byte is not zero
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), NotZeroByte(1, 14));

        let result = parser::parse_entries(
            &parser,
            4,
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 1, 0, // the third byte is not zero
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), NotZeroByte(1, 15));

        let result = parser::parse_entries(
            &parser,
            4,
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 1, // the fourth byte is not zero
                0, 0, 0, 0, //
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), NotZeroByte(1, 16));

        let result = parser::parse_entries(
            &parser,
            4,
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                1, 0, 0, 0, // the first byte is not zero
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), NotZeroByte(1, 17));

        let result = parser::parse_entries(
            &parser,
            4,
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 1, 0, 0, // the second byte is not zero
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), NotZeroByte(1, 18));

        let result = parser::parse_entries(
            &parser,
            4,
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 1, 0, // the third byte is not zero
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), NotZeroByte(1, 19));

        let result = parser::parse_entries(
            &parser,
            4,
            vec![
                2, 1, 0, 0, //
                0, 2, 0, 0, //
                192, 0, 2, 100, //
                0, 0, 0, 0, //
                0, 0, 0, 1, // the fourth byte is not zero
                4, 3, 2, 1, //
            ]
            .as_slice(),
        );
        assert_eq!(result.unwrap_err(), NotZeroByte(1, 20));
    }
}
