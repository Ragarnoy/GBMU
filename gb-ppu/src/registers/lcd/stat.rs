use super::Register;
use crate::drawing::Mode;
use crate::error::PPUResult;

use std::convert::TryInto;

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
        self.bits.lyc_eq_ly_interrupt() != 0
    }
    pub fn set_lyc_eq_ly_interrupt(&mut self, flag: bool) {
        self.bits.set_lyc_eq_ly_interrupt(if flag { 1 } else { 0 })
    }

    pub fn mode_2_interrupt(&self) -> bool {
        self.bits.mode_2_interrupt() != 0
    }
    pub fn set_mode_2_interrupt(&mut self, flag: bool) {
        self.bits.set_mode_2_interrupt(if flag { 1 } else { 0 })
    }

    pub fn mode_1_interrupt(&self) -> bool {
        self.bits.mode_1_interrupt() != 0
    }
    pub fn set_mode_1_interrupt(&mut self, flag: bool) {
        self.bits.set_mode_1_interrupt(if flag { 1 } else { 0 })
    }

    pub fn mode_0_interrupt(&self) -> bool {
        self.bits.mode_0_interrupt() != 0
    }
    pub fn set_mode_0_interrupt(&mut self, flag: bool) {
        self.bits.set_mode_0_interrupt(if flag { 1 } else { 0 })
    }

    pub fn lyc_eq_ly(&self) -> bool {
        self.bits.lyc_eq_ly() != 0
    }
    pub fn set_lyc_eq_ly(&mut self, flag: bool) {
        self.bits.set_lyc_eq_ly(if flag { 1 } else { 0 })
    }

    pub fn mode(&self) -> PPUResult<Mode> {
        self.bits.mode().try_into()
    }
    pub fn set_mode(&mut self, mode: Mode) {
        self.bits.set_mode(mode.into())
    }
}

impl From<u8> for Stat {
    fn from(byte: u8) -> Stat {
        Stat { bits: byte.into() }
    }
}

impl From<Stat> for u8 {
    fn from(register: Stat) -> u8 {
        register.bits.into()
    }
}

impl Register for Stat {
    const WRITE_BITS: u8 = 0b0111_1000;
}
