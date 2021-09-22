use anyhow::Result;

pub type RegisterMap = (String, RegisterValue);

impl From<RegisterValue> for u16 {
    fn from(input: RegisterValue) -> Self {
        match input {
            RegisterValue::U8(x) => x as u16,
            RegisterValue::U16(x) => x,
        }
    }
}

#[derive(Clone, Copy)]
pub enum RegisterValue {
    U8(u8),
    U16(u16),
}

impl From<u8> for RegisterValue {
    fn from(input: u8) -> Self {
        Self::U8(input)
    }
}

impl From<u16> for RegisterValue {
    fn from(input: u16) -> Self {
        Self::U16(input)
    }
}

pub trait MemoryDebugOperations {
    fn read(&self, index: usize) -> u8;
}

pub trait RegisterDebugOperations {
    fn cpu_get(&self, key: &str) -> Result<RegisterValue>;

    fn ppu_get(&self, key: &str) -> Result<RegisterValue>;

    fn io_get(&self, key: &str) -> Result<RegisterValue>;

    fn cpu_registers(&self) -> Vec<RegisterMap>;

    fn ppu_registers(&self) -> Vec<RegisterMap>;

    fn io_registers(&self) -> Vec<RegisterMap>;
}
