type RegisterMap = (u16, String);

pub trait DebugRW {
    type RegisterIter: Iterator<Item=RegisterMap>;

    fn read(&self, index: usize) -> u8;

    fn write(&mut self, index: usize, value: u8);

    fn register_iter(&self) -> Self::RegisterIter;
}
