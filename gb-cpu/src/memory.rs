use super::registers::Registers;
use super::wram::Wram;

#[derive(Debug, Default)]
pub struct Memory {
    pub registers: Registers,
    pub wram: Wram,
}

