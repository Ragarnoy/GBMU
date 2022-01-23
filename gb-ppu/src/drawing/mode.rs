use crate::error::{PPUError, PPUResult};
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Mode {
    HBlank,
    VBlank,
    OAMFetch,
    PixelDrawing,
}

impl TryFrom<u8> for Mode {
    type Error = PPUError;

    fn try_from(byte: u8) -> PPUResult<Mode> {
        match byte {
            0 => Ok(Mode::HBlank),
            1 => Ok(Mode::VBlank),
            2 => Ok(Mode::OAMFetch),
            3 => Ok(Mode::PixelDrawing),
            _ => Err(PPUError::OutOfBound {
                value: byte as usize,
                min_bound: 0,
                max_bound: 4,
            }),
        }
    }
}

impl From<Mode> for u8 {
    fn from(mode: Mode) -> u8 {
        match mode {
            Mode::HBlank => 0,
            Mode::VBlank => 1,
            Mode::OAMFetch => 2,
            Mode::PixelDrawing => 3,
        }
    }
}
