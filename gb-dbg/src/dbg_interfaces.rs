use anyhow::Result;

pub type RegisterMap = (String, RegisterType);

pub enum RegisterType {
    U8(u8),
    U16(u16),
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
