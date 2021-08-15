use super::super::area::Bits8;
use super::super::pc::NextPc;
use crate::bus::Bus;
use crate::error::Error;
use crate::memory::Memory;
use crate::soc::cpu::registers::Registers;
use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegNum8bit {
    B = 0x06,
    C = 0x0E,
    D = 0x16,
    E = 0x1E,
    H = 0x26,
    L = 0x2E,
}

impl<'a> LoadRegNum8bit {
    pub fn proceed(self, registers: &'a mut Registers, memory: &'a Memory) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            match self {
                LoadRegNum8bit::B => registers.set(Bits8::B, byte),
                LoadRegNum8bit::C => registers.set(Bits8::C, byte),
                LoadRegNum8bit::D => registers.set(Bits8::D, byte),
                LoadRegNum8bit::E => registers.set(Bits8::E, byte),
                LoadRegNum8bit::H => registers.set(Bits8::H, byte),
                LoadRegNum8bit::L => registers.set(Bits8::L, byte),
            };
            Ok(8)
        }
        else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}
