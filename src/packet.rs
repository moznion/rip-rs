use crate::packet::PacketError::VersionInHeaderConflicted;
use crate::serializer::{Serializable, SerializeError};
use crate::{header, v1, v2, version};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("version in the header conflicted")]
    VersionInHeaderConflicted,
}

#[derive(PartialEq, Debug)]
pub struct Packet<T> {
    header: header::Header,
    entries: Vec<T>,
}

impl<T> Packet<T> {
    fn new(header: header::Header, entries: Vec<T>) -> Self {
        Packet { header, entries }
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
        Ok(Packet::new(header, entries))
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
        Ok(Packet::new(header, entries))
    }
}

impl<T: Serializable> Serializable for Packet<T> {
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        let mut entries_bytes = vec![];

        for entry in &self.entries {
            entries_bytes.extend(entry.to_bytes()?);
        }

        Ok([self.header.to_bytes()?, entries_bytes].concat())
    }
}
