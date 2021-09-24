use super::Register;
use crate::error::PPUResult;
use crate::mode::PPUMode;

use std::convert::TryInto;

use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2},
};

#[bitfield]
#[derive(Clone, Copy, Debug, Default)]
struct StatBits {
    #[allow(dead_code)]
    pub unused_bit: B1,
    pub lyc_eq_ly_interrupt: B1,
    pub mode_2_interrupt: B1,
    pub mode_1_interrupt: B1,
    pub mode_0_interrupt: B1,
    pub lyc_eq_ly: B1,
    pub mode: B2,
}

#[derive(Default, Clone, Copy)]
pub struct Stat {
    bits: StatBits,
}

impl Stat {
    pub const SIZE: usize = 1;

    pub fn new() -> Self {
        Stat {
            bits: StatBits::new(),
        }
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

    pub fn mode(&self) -> PPUResult<PPUMode> {
        self.bits.mode().try_into()
    }
    pub fn set_mode(&mut self, mode: PPUMode) {
        self.bits.set_mode(mode.into())
    }
}

impl From<u8> for StatBits {
    fn from(byte: u8) -> StatBits {
        StatBits::from_bytes([byte])
    }
}

impl From<u8> for Stat {
    fn from(byte: u8) -> Stat {
        Stat { bits: byte.into() }
    }
}

impl From<StatBits> for u8 {
    fn from(bits: StatBits) -> u8 {
        bits.into_bytes()[0]
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
