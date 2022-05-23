use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    InvalidCartridgeType(u8),
    InvalidDestinationCode(u8),
    InvalidCgbFlag(u8),
    InvalidSgbFlag(u8),
    InvalidNewLicenseCode(String),
    InvalidOldLicenseCode(u8),
    InvalidRamSize(u8),
    InvalidRomSize(u8),
    InvalidUtf8(std::string::FromUtf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidCartridgeType(v) => write!(f, "Invalid cartridge type {:02x}", v),
            Error::InvalidDestinationCode(v) => write!(f, "Invalid destination code {:02x}", v),
            Error::InvalidCgbFlag(v) => write!(f, "Invalid cgb flag {:02x}", v),
            Error::InvalidSgbFlag(v) => write!(f, "Invalid sgb flag {:02x}", v),
            Error::InvalidNewLicenseCode(v) => write!(f, "Invalid new license code {}", v),
            Error::InvalidOldLicenseCode(v) => write!(f, "Invalid old license code {:02x}", v),
            Error::InvalidRamSize(v) => write!(f, "Invalid ram size {:02x}", v),
            Error::InvalidRomSize(v) => write!(f, "Invalid rom size {:02x}", v),
            Error::InvalidUtf8(v) => {
                write!(f, "invalid utf8 for {:?}: {}", v.as_bytes(), v.utf8_error())
            }
        }
    }
}

impl std::convert::From<std::string::FromUtf8Error> for Error {
    fn from(v: std::string::FromUtf8Error) -> Self {
        Self::InvalidUtf8(v)
    }
}

impl std::error::Error for Error {}
