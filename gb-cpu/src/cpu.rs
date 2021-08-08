use super::registers::Registers;
use crate::memory::Memory;
use super::error::Error;

#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    fn next(&mut self, memory: &Memory) -> Result<u8, Error> {
        match memory.read(self.registers.pc){
            Ok(byte) => {
                self.registers.next_pc();
                Ok(byte)
            }
            Err(_) => Err(Error::InvalidPC(self.registers.pc))
        }
    }

    pub fn step(&mut self, memory: &Memory) -> u32 {
        let _opcode = self.next(memory);

        //do opcode, return cycles taken
        1
    }
}
