use crate::serializer::{Serializable, SerializeError};
use crate::{address_family, ipv4, metric, parser::PacketParsable, parser::ParseError, route_tag};
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

impl Entry {
    pub fn new(
        address_family_identifier: address_family::Identifier,
        route_tag: u16,
        ip_address: Ipv4Addr,
        subnet_mask: Ipv4Addr,
        next_hop: Ipv4Addr,
        metric: u32,
    ) -> Self {
        Entry {
            address_family_identifier,
            route_tag,
            ip_address,
            subnet_mask,
            next_hop,
            metric,
        }
    }

    pub fn get_address_family_identifier(&self) -> address_family::Identifier {
        self.address_family_identifier
    }

    pub fn get_route_tag(&self) -> u16 {
        self.route_tag
    }

    pub fn get_ip_address(&self) -> Ipv4Addr {
        self.ip_address
    }

    pub fn get_subnet_mask(&self) -> Ipv4Addr {
        self.subnet_mask
    }

    pub fn get_next_hop(&self) -> Ipv4Addr {
        self.next_hop
    }

    pub fn get_metric(&self) -> u32 {
        self.metric
    }
}

impl Serializable for Entry {
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        Ok([
            self.get_address_family_identifier().to_bytes()?,
            route_tag::to_bytes(self.get_route_tag())?,
            ipv4::to_bytes(self.get_ip_address())?,
            ipv4::to_bytes(self.get_subnet_mask())?,
            ipv4::to_bytes(self.get_next_hop())?,
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
        let (route_tag, cursor) = route_tag::parse(cursor, bytes)?;
        let (ip_address, cursor) = ipv4::parse(cursor, bytes)?;
        let (subnet_mask, cursor) = ipv4::parse(cursor, bytes)?;
        let (next_hop, cursor) = ipv4::parse(cursor, bytes)?;
        let (metric, cursor) = metric::parse(cursor, bytes)?;

        Ok((
            Entry::new(
                address_family_identifier,
                route_tag,
                ip_address,
                subnet_mask,
                next_hop,
                metric,
            ),
            cursor,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::v2::{EntriesParser, Entry};
    use crate::{address_family, parser};
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_packet_for_single_entry() {
        let parser = EntriesParser {};
        let result = parser::parse_entries(
            &parser,
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
        let result = parser::parse_entries(
            &parser,
            4,
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
