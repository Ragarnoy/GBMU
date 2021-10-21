use gb_cpu::registers::Registers;
use std::str::FromStr;

pub enum Reg16 {
    PC,
    SP,
    DE,
    HL,
}

impl Reg16 {
    pub fn read_corresponding_regs(&self, regs: &Registers) -> u16 {
        match self {
            Reg16::PC => regs.pc,
            Reg16::SP => regs.sp,
            Reg16::DE => regs.de,
            Reg16::HL => regs.hl,
        }
    }

    pub fn write_corresponding_regs(&self, regs: &mut Registers, value: u16) {
        match self {
            Reg16::PC => regs.pc = value,
            Reg16::SP => regs.sp = value,
            Reg16::DE => regs.de = value,
            Reg16::HL => regs.hl = value,
        }
    }
}

impl FromStr for Reg16 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PC" => Ok(Reg16::PC),
            "DE" => Ok(Reg16::DE),
            "SP" => Ok(Reg16::SP),
            "HL" => Ok(Reg16::HL),
            _ => Err(format!("invalid 16-bits register name {}", s)),
        }
    }
}
