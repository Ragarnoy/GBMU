use anyhow::anyhow;
use gb_dbg::dbg_interfaces::{
    DebugOperations, MemoryDebugOperations, RegisterDebugOperations, RegisterMap, RegisterValue,
};

pub struct Iter<'a> {
    count: u32,
    registers: &'a Game,
}

impl<'a> Iterator for Iter<'a> {
    type Item = RegisterMap;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        match self.count {
            1 => Some(("A".to_owned(), RegisterValue::from(self.registers.a))),
            2 => Some(("B".to_owned(), RegisterValue::from(self.registers.b))),
            3 => Some(("C".to_owned(), RegisterValue::from(self.registers.c))),
            4 => Some(("D".to_owned(), RegisterValue::from(self.registers.d))),
            5 => Some(("E".to_owned(), RegisterValue::from(self.registers.e))),
            6 => Some(("F".to_owned(), RegisterValue::from(self.registers.f))),
            7 => Some(("PC".to_owned(), RegisterValue::from(self.registers.pc))),
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
impl From<&Game> for Vec<RegisterMap> {
    fn from(registers: &Game) -> Self {
        vec![
            ("A".to_owned(), RegisterValue::from(registers.a)),
            ("B".to_owned(), RegisterValue::from(registers.b)),
            ("C".to_owned(), RegisterValue::from(registers.c)),
            ("D".to_owned(), RegisterValue::from(registers.d)),
            ("E".to_owned(), RegisterValue::from(registers.e)),
            ("F".to_owned(), RegisterValue::from(registers.f)),
            ("PC".to_owned(), RegisterValue::from(registers.pc)),
        ]
    }
}

impl RegisterDebugOperations for Game {
    fn cpu_get(&self, key: &str) -> anyhow::Result<RegisterValue> {
        match key.to_uppercase().as_str() {
            "A" => Ok(RegisterValue::from(self.a)),
            "B" => Ok(RegisterValue::from(self.b)),
            "C" => Ok(RegisterValue::from(self.c)),
            "D" => Ok(RegisterValue::from(self.d)),
            "E" => Ok(RegisterValue::from(self.e)),
            "F" => Ok(RegisterValue::from(self.f)),
            "PC" => Ok(RegisterValue::from(self.pc)),
            _ => Err(anyhow!("Not a valid register!")),
        }
    }

    fn ppu_get(&self, key: &str) -> anyhow::Result<RegisterValue> {
        self.cpu_get(key)
    }

    fn io_get(&self, key: &str) -> anyhow::Result<RegisterValue> {
        self.cpu_get(key)
    }

    fn cpu_registers(&self) -> Vec<RegisterMap> {
        self.into()
    }

    fn ppu_registers(&self) -> Vec<RegisterMap> {
        self.into()
    }

    fn io_registers(&self) -> Vec<RegisterMap> {
        self.into()
    }
}
