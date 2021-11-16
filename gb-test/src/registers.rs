use gb_cpu::registers::Registers;
use std::str::FromStr;

pub enum Reg16 {
    PC,
    SP,
    BC,
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
            Reg16::BC => regs.bc,
        }
    }

    pub fn write_corresponding_regs(&self, regs: &mut Registers, value: u16) {
        match self {
            Reg16::PC => regs.pc = value,
            Reg16::SP => regs.sp = value,
            Reg16::DE => regs.de = value,
            Reg16::HL => regs.hl = value,
            Reg16::BC => regs.bc = value,
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
            "BC" => Ok(Reg16::BC),
            _ => Err(format!("invalid 16-bits register name {}", s)),
        }
    }
}

pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Reg8 {
    pub fn read_corresponding_regs(&self, regs: &Registers) -> u8 {
        use gb_cpu::interfaces::Read8BitsReg;

        match self {
            Reg8::A => regs.a(),
            Reg8::B => regs.b(),
            Reg8::C => regs.c(),
            Reg8::D => regs.d(),
            Reg8::E => regs.e(),
            Reg8::H => regs.h(),
            Reg8::L => regs.l(),
        }
    }

    pub fn write_corresponding_regs(&self, regs: &mut Registers, value: u8) {
        use gb_cpu::interfaces::Write8BitsReg;

        match self {
            Reg8::A => regs.set_a(value),
            Reg8::B => regs.set_b(value),
            Reg8::C => regs.set_c(value),
            Reg8::D => regs.set_d(value),
            Reg8::E => regs.set_e(value),
            Reg8::H => regs.set_h(value),
            Reg8::L => regs.set_l(value),
        }
    }
}

impl FromStr for Reg8 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Reg8::A),
            "B" => Ok(Reg8::B),
            "C" => Ok(Reg8::C),
            "D" => Ok(Reg8::D),
            "E" => Ok(Reg8::E),
            "H" => Ok(Reg8::H),
            "L" => Ok(Reg8::L),
            _ => Err(format!("invalid 8-bits register name {}", s)),
        }
    }
}
