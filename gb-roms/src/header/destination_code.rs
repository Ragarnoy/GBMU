use super::error::Error;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DestinationCode {
    Japan,
    Other,
}

#[cfg(test)]
impl Default for DestinationCode {
    fn default() -> Self {
        Self::Other
    }
}

impl TryFrom<u8> for DestinationCode {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(DestinationCode::Japan),
            0x01 => Ok(DestinationCode::Other),
            _ => Err(Error::InvalidDestinationCode(v)),
        }
    }
}

#[test]
fn test_convert_destination_code() {
    assert_eq!(DestinationCode::try_from(0x00), Ok(DestinationCode::Japan));
    assert_eq!(DestinationCode::try_from(0x01), Ok(DestinationCode::Other));
}
