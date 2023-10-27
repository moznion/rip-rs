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
        let command_bytes = self.get_command().to_bytes()?;
        let version_bytes = self.get_version().to_bytes()?;
        Ok([command_bytes, version_bytes, vec![0, 0]].concat())
    }
}

#[cfg(test)]
mod tests {
    use crate::header::Header;
    use crate::serializer::Serializable;
    use crate::{command, version};

    #[test]
    fn test_to_bytes() {
        assert_eq!(
            Header::new(command::Kind::Request, version::Version::Version2)
                .to_bytes()
                .unwrap(),
            vec![0x01, 0x02, 0x00, 0x00]
        );
    }
}
