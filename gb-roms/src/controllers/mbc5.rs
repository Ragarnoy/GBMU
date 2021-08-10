use gb_cpu::address_bus::{Address, Area, Error, FileOperation};

pub struct MBC5 {}

impl FileOperation for MBC5 {
    fn read(&self, addr: Address) -> Result<u8, Error> {
        unimplemented!("no read for mbc5")
    }

    fn write(&mut self, v: u8, addr: Address) -> Result<(), Error> {
        unimplemented!("no write for mbc5")
    }
}
