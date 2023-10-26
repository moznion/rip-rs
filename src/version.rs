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
