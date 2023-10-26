use crate::{command, version};

#[derive(PartialEq, Debug)]
pub struct Header {
    command: command::Kind,
    version: version::Version,
}

impl Header {
    pub fn new(command: command::Kind, version: version::Version) -> Self {
        Header { command, version }
    }

    pub fn get_command(&self) -> command::Kind {
        self.command
    }

    pub fn get_version(&self) -> version::Version {
        self.version
    }
}
