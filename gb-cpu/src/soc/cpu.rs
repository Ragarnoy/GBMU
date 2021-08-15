mod area;
mod flags;
mod opcodes;
mod pc;
pub mod registers;

use crate::error::Error;
use crate::memory::Memory;
use registers::Registers;
use opcodes::LoadRegNum8bit;
use std::convert::TryFrom;

#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    pub fn step(&mut self, memory: &Memory) -> Result<u32, Error> {
        //let opcode = self.next(memory);
        let opcode = 0x06;

        if let Ok(load) = LoadRegNum8bit::try_from(opcode) {
            load.proceed(&mut self.registers, memory)
        } else {
            Err(Error::Unimplemented)
        }
    }
}
