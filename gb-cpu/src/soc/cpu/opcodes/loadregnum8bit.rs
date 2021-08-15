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
    pub fn proceed(self, registers: &'a mut Registers, memory: &'a mut Memory) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let result = match self {
                LoadRegNum8bit::B => memory.set(byte.into(), registers.get(Bits8::B)),
                LoadRegNum8bit::C => memory.set(byte.into(), registers.get(Bits8::C)),
                LoadRegNum8bit::D => memory.set(byte.into(), registers.get(Bits8::D)),
                LoadRegNum8bit::E => memory.set(byte.into(), registers.get(Bits8::E)),
                LoadRegNum8bit::H => memory.set(byte.into(), registers.get(Bits8::H)),
                LoadRegNum8bit::L => memory.set(byte.into(), registers.get(Bits8::L)),
            };
            match result {
                Ok(_) => Ok(8),
                Err(e) => Err(e),
            }
        }
        else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}
