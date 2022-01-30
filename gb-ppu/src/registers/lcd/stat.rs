use super::Register;
use crate::drawing::Mode;
use crate::error::PPUResult;

use std::convert::TryInto;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Stat {
    bits: u8,
}

impl Stat {
    pub const SIZE: usize = 1;
    const MODE: u8 = 0b11;
    const LYC_EQ_LY: u8 = 0b100;
    const MODE_0_INTERRUPT: u8 = 0b1000;
    const MODE_1_INTERRUPT: u8 = 0b1_0000;
    const MODE_2_INTERRUPT: u8 = 0b10_0000;
    const LYC_EQ_LY_INTERRUPT: u8 = 0b100_0000;

    pub fn new() -> Self {
        Stat { bits: 0x80 }
    }

    pub fn lyc_eq_ly_interrupt(&self) -> bool {
        self.bits & Self::LYC_EQ_LY_INTERRUPT != 0
    }

    pub fn mode_2_interrupt(&self) -> bool {
        self.bits & Self::MODE_2_INTERRUPT != 0
    }

    pub fn mode_1_interrupt(&self) -> bool {
        self.bits & Self::MODE_1_INTERRUPT != 0
    }

    pub fn mode_0_interrupt(&self) -> bool {
        self.bits & Self::MODE_0_INTERRUPT != 0
    }

    pub fn lyc_eq_ly(&self) -> bool {
        self.bits & Self::LYC_EQ_LY != 0
    }
    pub fn set_lyc_eq_ly(&mut self, flag: bool) {
        if flag {
            self.bits |= Self::LYC_EQ_LY;
        } else {
            self.bits &= !Self::LYC_EQ_LY;
        }
    }

    pub fn mode(&self) -> PPUResult<Mode> {
        (self.bits & Self::MODE).try_into()
    }
    pub fn set_mode(&mut self, mode: Mode) {
        let mode_byte: u8 = mode.into();
        self.bits = (self.bits & !Self::MODE) | (mode_byte & Self::MODE);
    }
}

impl From<u8> for Stat {
    fn from(byte: u8) -> Stat {
        Stat { bits: byte }
    }
}

impl From<Stat> for u8 {
    fn from(register: Stat) -> u8 {
        register.bits
    }
}

impl Register for Stat {
    const WRITE_BITS: u8 = 0b0111_1000;
}
