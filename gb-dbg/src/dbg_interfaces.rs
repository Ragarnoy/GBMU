use anyhow::Result;

pub type RegisterMap = (String, RegisterType);

impl From<RegisterType> for u16 {
    fn from(input: RegisterType) -> Self {
        match input {
            RegisterType::U8(x) => x as u16,
            RegisterType::U16(x) => x,
        }
    }
}

pub enum RegisterType {
    U8(u8),
    U16(u16),
}

impl From<u8> for RegisterType {
    fn from(input: u8) -> Self {
        Self::U8(input)
    }
}

impl From<u16> for RegisterType {
    fn from(input: u16) -> Self {
        Self::U16(input)
    }
}

pub trait RW {
    fn read(&self, index: usize) -> u8;

    fn write(&mut self, _index: usize, _value: u8) {
        // Default to Read-Only
    }
}

pub trait DebugRegister {
    type RegisterIter: Iterator<Item = RegisterMap>;

    fn get(&self, key: &str) -> Result<RegisterType>;

    fn register_iter(&self) -> Self::RegisterIter;
}
