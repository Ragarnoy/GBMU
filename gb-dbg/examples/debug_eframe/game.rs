use gb_dbg::dbg_interfaces::{
    CpuRegs, DebugOperations, IORegs, MemoryDebugOperations, PpuRegs, RegisterDebugOperations,
    RegisterMap, RegisterValue,
};

pub struct Iter<'a> {
    count: u32,
    registers: &'a Game,
}

impl<'a> Iterator for Iter<'a> {
    type Item = RegisterMap<CpuRegs>;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        match self.count {
            1 => Some(RegisterMap(
                CpuRegs::AF,
                RegisterValue::from(self.registers.a),
            )),
            2 => Some(RegisterMap(
                CpuRegs::BC,
                RegisterValue::from(self.registers.b),
            )),
            3 => Some(RegisterMap(
                CpuRegs::DE,
                RegisterValue::from(self.registers.c),
            )),
            4 => Some(RegisterMap(
                CpuRegs::HL,
                RegisterValue::from(self.registers.d),
            )),
            5 => Some(RegisterMap(
                CpuRegs::SP,
                RegisterValue::from(self.registers.e),
            )),
            6 => Some(RegisterMap(
                CpuRegs::PC,
                RegisterValue::from(self.registers.pc),
            )),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub pc: u16,
    pub memory: Vec<u8>,
}

impl DebugOperations for Game {}

impl MemoryDebugOperations for Game {
    fn read(&self, index: u16) -> u8 {
        *self.memory.get(index as usize).unwrap()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            a: 0xFF,
            b: 0x1F,
            c: 0x22,
            d: 0x3F,
            e: 4,
            f: 8,
            pc: 500,
            memory: vec![0xFFu8; u16::MAX as usize],
        }
    }
}

impl Game {
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            count: 0,
            registers: self,
        }
    }
}

//TODO Temporary for now it looks like ass
impl From<&Game> for Vec<RegisterMap<CpuRegs>> {
    fn from(registers: &Game) -> Self {
        vec![
            RegisterMap(CpuRegs::AF, RegisterValue::from(registers.a)),
            RegisterMap(CpuRegs::BC, RegisterValue::from(registers.b)),
            RegisterMap(CpuRegs::DE, RegisterValue::from(registers.d)),
            RegisterMap(CpuRegs::HL, RegisterValue::from(registers.f)),
            RegisterMap(CpuRegs::PC, RegisterValue::from(registers.pc)),
        ]
    }
}

impl RegisterDebugOperations for Game {
    fn cpu_get(&self, key: CpuRegs) -> anyhow::Result<RegisterValue> {
        match key {
            CpuRegs::AF => Ok(RegisterValue::from(self.a)),
            CpuRegs::BC => Ok(RegisterValue::from(self.b)),
            CpuRegs::DE => Ok(RegisterValue::from(self.d)),
            CpuRegs::HL => Ok(RegisterValue::from(self.f)),
            CpuRegs::SP => Ok(RegisterValue::from(self.c)),
            CpuRegs::PC => Ok(RegisterValue::from(self.pc)),
        }
    }

    fn ppu_get(&self, _key: PpuRegs) -> anyhow::Result<RegisterValue> {
        unimplemented!("only testing with cpu")
    }

    fn io_get(&self, _key: IORegs) -> anyhow::Result<RegisterValue> {
        unimplemented!("only testing with cpu")
    }

    fn cpu_registers(&self) -> Vec<RegisterMap<CpuRegs>> {
        self.into()
    }

    fn ppu_registers(&self) -> Vec<RegisterMap<PpuRegs>> {
        unimplemented!("only testing with cpu")
    }

    fn io_registers(&self) -> Vec<RegisterMap<IORegs>> {
        unimplemented!("only testing with cpu")
    }
}
