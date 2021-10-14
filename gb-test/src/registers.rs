use gb_cpu::registers::Registers;
use std::str::FromStr;

pub enum Reg16 {
    PC,
}

impl Reg16 {
    pub fn read_corresponding_regs(&self, regs: &Registers) -> u16 {
        match self {
            Reg16::PC => regs.pc,
        }
    }
}

impl FromStr for Reg16 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PC" => Ok(Reg16::PC),
            _ => Err(format!("invalid 16-bits register name {}", s)),
        }
    }
}
