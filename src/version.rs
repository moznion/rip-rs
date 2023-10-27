use crate::serializer::SerializeError::UnknownVersion;
use crate::serializer::{Serializable, SerializeError};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Version {
    MustBeDiscarded, // RFC1058
    Version1,        // RFC1058
    Version2,        // RFC2453
    Unknown,
}

impl Version {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Version::MustBeDiscarded,
            1 => Version::Version1,
            2 => Version::Version2,
            _ => Version::Unknown,
        }
    }

    pub fn to_u8(&self) -> Option<u8> {
        match self {
            Version::MustBeDiscarded => Some(0),
            Version::Version1 => Some(1),
            Version::Version2 => Some(2),
            Version::Unknown => None,
        }
    }
}

impl Serializable for Version {
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        match self.to_u8() {
            Some(byte) => Ok(vec![byte]),
            None => Err(UnknownVersion),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::serializer::Serializable;
    use crate::serializer::SerializeError::UnknownVersion;
    use crate::version;
    use crate::version::Version::{MustBeDiscarded, Unknown, Version1, Version2};

    #[test]
    fn test_from_u8() {
        assert_eq!(version::Version::from_u8(0), MustBeDiscarded);
        assert_eq!(version::Version::from_u8(1), Version1);
        assert_eq!(version::Version::from_u8(2), Version2);
        assert_eq!(version::Version::from_u8(3), Unknown);
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(MustBeDiscarded.to_bytes().unwrap(), vec![0x00]);
        assert_eq!(Version1.to_bytes().unwrap(), vec![0x01]);
        assert_eq!(Version2.to_bytes().unwrap(), vec![0x02]);
        assert_eq!(Unknown.to_bytes().unwrap_err(), UnknownVersion);
    }
}
