use crate::serializer::{Serializable, SerializeError};
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

impl Serializable for Header {
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        let command_bytes = self.command.to_bytes()?;
        let version_bytes = self.version.to_bytes()?;
        Ok([command_bytes, version_bytes, vec![0, 0]].concat())
    }
}
