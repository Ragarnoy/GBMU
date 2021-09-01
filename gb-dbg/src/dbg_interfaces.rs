type RegisterMap = (String, RegisterType);

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

    fn get<T: Into<u16>>(&self, key: &str) -> T;

    fn set<T: Into<u16>>(&mut self, _key: &str, _value: T) {
        // Default to Read-Only
    }

    fn register_iter(&self) -> Self::RegisterIter;
}
