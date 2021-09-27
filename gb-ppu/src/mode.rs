use crate::error::{PPUError, PPUResult};
use std::convert::TryFrom;

pub enum PPUMode {
    HBlank,
    VBlank,
    OAMScan,
    Drawing,
}

impl TryFrom<u8> for PPUMode {
    type Error = PPUError;

    fn try_from(byte: u8) -> PPUResult<PPUMode> {
        match byte {
            0 => Ok(PPUMode::HBlank),
            1 => Ok(PPUMode::VBlank),
            2 => Ok(PPUMode::OAMScan),
            3 => Ok(PPUMode::Drawing),
            _ => Err(PPUError::OutOfBound {
                value: byte as usize,
                min_bound: 0,
                max_bound: 4,
            }),
        }
    }
}

impl From<PPUMode> for u8 {
    fn from(mode: PPUMode) -> u8 {
        match mode {
            PPUMode::HBlank => 0,
            PPUMode::VBlank => 1,
            PPUMode::OAMScan => 2,
            PPUMode::Drawing => 3,
        }
    }
}
