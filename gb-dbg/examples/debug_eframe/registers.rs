use anyhow::anyhow;
use gb_dbg::dbg_interfaces::{DebugRegister, RegisterMap, RegisterType};

pub struct Iter<'a> {
    count: u32,
    registers: &'a Registers,
}

impl<'a> Iterator for Iter<'a> {
    type Item = RegisterMap;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        match self.count {
            1 => Some(("A".to_owned(), RegisterType::from(self.registers.a))),
            2 => Some(("B".to_owned(), RegisterType::from(self.registers.b))),
            3 => Some(("C".to_owned(), RegisterType::from(self.registers.c))),
            4 => Some(("D".to_owned(), RegisterType::from(self.registers.d))),
            5 => Some(("E".to_owned(), RegisterType::from(self.registers.e))),
            6 => Some(("F".to_owned(), RegisterType::from(self.registers.f))),
            _ => None,
        }
    }
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0xFF,
            b: 0x1F,
            c: 0x22,
            d: 0x3F,
            e: 4,
            f: 8,
        }
    }
}

impl Registers {
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            count: 0,
            registers: self,
        }
    }
}

impl<'a> DebugRegister for &'a Registers {
    type RegisterIter = Iter<'a>;

    fn get(&self, key: &str) -> anyhow::Result<RegisterType> {
        match key {
            "A" => Ok(RegisterType::from(self.a)),
            "B" => Ok(RegisterType::from(self.b)),
            "C" => Ok(RegisterType::from(self.c)),
            "D" => Ok(RegisterType::from(self.d)),
            "E" => Ok(RegisterType::from(self.e)),
            "F" => Ok(RegisterType::from(self.f)),
            _ => Err(anyhow!("Not a valid register!")),
        }
    }

    fn register_iter(&self) -> Self::RegisterIter {
        self.iter()
    }
}
