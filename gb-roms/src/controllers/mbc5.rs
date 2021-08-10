use crate::header::size::{RamSize, RomSize};
use gb_cpu::address_bus::{Address, Area, Error, FileOperation};
use std::io::{self, Read};

pub struct MBC5 {}

impl MBC5 {
    /// initialize the controller using a file as the rom
    pub fn from_file(
        mut file: impl Read,
        ram_size: RamSize,
        rom_size: RomSize,
    ) -> Result<MBC5, io::Error> {
        let ctl = MBC5::empty(ram_size, rom_size);

        Ok(ctl)
    }

    /// empty return an empty initialized controller
    pub fn empty(ram_size: RamSize, rom_size: RomSize) -> MBC5 {
        unimplemented!("cannot create mbc5 controller")
    }
}

impl FileOperation for MBC5 {
    fn read(&self, addr: Address) -> Result<u8, Error> {
        unimplemented!("no read for mbc5")
    }

    fn write(&mut self, v: u8, addr: Address) -> Result<(), Error> {
        unimplemented!("no write for mbc5")
    }
}
