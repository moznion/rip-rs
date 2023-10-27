use crate::parsed::Parsed;
use crate::serializer::SerializeError;
use crate::{byte_reader, parser::ParseError};
use std::net::Ipv4Addr;

pub(crate) fn parse(cursor: usize, bytes: &[u8]) -> Result<Parsed<Ipv4Addr>, ParseError> {
    let (ipaddr_first_octet, cursor) = byte_reader::read(cursor, bytes)?;
    let (ipaddr_second_octet, cursor) = byte_reader::read(cursor, bytes)?;
    let (ipaddr_third_octet, cursor) = byte_reader::read(cursor, bytes)?;
    let (ipaddr_fourth_octet, cursor) = byte_reader::read(cursor, bytes)?;

    Ok(Parsed::new(
        Ipv4Addr::new(
            ipaddr_first_octet,
            ipaddr_second_octet,
            ipaddr_third_octet,
            ipaddr_fourth_octet,
        ),
        cursor,
    ))
}

pub(crate) fn to_bytes(ipv4: Ipv4Addr) -> Result<Vec<u8>, SerializeError> {
    Ok(ipv4.octets().to_vec())
}
