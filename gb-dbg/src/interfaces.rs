type RegisterMap = (String, u8);

pub trait RW {
    fn read(&self, index: usize) -> u8;

    fn write(&mut self, index: usize, value: u8);
}

pub trait DebugRegister: RW {
    type RegisterIter: Iterator<Item = RegisterMap>;

    fn register_iter(&self) -> Self::RegisterIter;
}
