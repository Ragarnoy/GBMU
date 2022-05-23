use super::error::Error;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CgbFlag {
    // Rom support Cgb + old Game Boys
    RetroCompatible,
    // Rom support Cgb only
    CgbOnly,
}

#[cfg(test)]
impl Default for CgbFlag {
    fn default() -> Self {
        Self::RetroCompatible
    }
}

impl TryFrom<u8> for CgbFlag {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x80 => Ok(CgbFlag::RetroCompatible),
            0xC0 => Ok(CgbFlag::CgbOnly),
            _ => Err(Error::InvalidCgbFlag(v)),
        }
    }
}

#[test]
fn test_convert_cgb_flag() {
    assert_eq!(CgbFlag::try_from(0x80), Ok(CgbFlag::RetroCompatible));
    assert_eq!(CgbFlag::try_from(0xC0), Ok(CgbFlag::CgbOnly));
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SgbFlag {
    Unsupported,
    Supported,
}

#[cfg(test)]
impl Default for SgbFlag {
    fn default() -> Self {
        Self::Supported
    }
}

impl TryFrom<u8> for SgbFlag {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(SgbFlag::Unsupported),
            0x03 => Ok(SgbFlag::Supported),
            _ => Err(Error::InvalidSgbFlag(v)),
        }
    }
}

#[test]
fn test_convert_sgb_flag() {
    assert_eq!(SgbFlag::try_from(0x00), Ok(SgbFlag::Unsupported));
    assert_eq!(SgbFlag::try_from(0x03), Ok(SgbFlag::Supported));
}
