use super::Register;
use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2},
};

#[bitfield]
#[derive(Clone, Copy, Debug, Default)]
struct StatBits {
    pub _unused_bit: B1,
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
