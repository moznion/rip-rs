use crate::header;

#[derive(PartialEq, Debug)]
pub struct Packet<T> {
    header: header::Header,
    entries: Vec<T>,
}

impl<T> Packet<T> {
    pub fn new(header: header::Header, entries: Vec<T>) -> Self {
        Packet { header, entries }
    }

    pub fn get_header(&self) -> &header::Header {
        &self.header
    }

    pub fn get_entries(&self) -> &Vec<T> {
        &self.entries
    }
}
