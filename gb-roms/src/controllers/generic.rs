use gb_bus::{Address, Area, Error, FileOperation};
use serde::{Deserialize, Serialize};
use std::{
    io::{self, Read},
    rc::Rc,
};

use crate::Header;

use super::{new_controller_from_header, Controller};

pub struct Generic {
    controller: Rc<dyn Controller>,
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
}

impl Generic {
    /// Create an empty Generic MBC from an header
    fn new(header: Header) -> Self {
        let ctl = new_controller_from_header(header);
        let (rom_size, ram_size) = ctl.sizes();

        Self {
            controller: ctl,
            rom: ctl.create_rom(),
            ram: ctl.create_ram(),
        }
    }

    /// Create a Generic MBC from an header with is corresponding ROM data
    fn from_reader(header: Header, mut reader: impl Read) -> Result<Self, io::Error> {
        let mut mbc = Self::new(header);

        reader.read_exact(&mut mbc.rom)?;
        Ok(mbc)
    }

    fn save_state(&self) -> GenericState {
        GenericState {
            controller: self.controller.save_to_slice(),
            ram: self.ram.clone(),
        }
    }

    fn load_state(&mut self, state: GenericState) {
        self.ram = state.ram;
        self.controller.load_from_slice(&state.controller);
    }

    fn read_rom(&self, addr: u16) -> Result<u8, Error> {
        todo!()
    }

    fn write_rom(&self, addr: u16) -> Result<(), Error> {
        todo!()
    }

    fn read_ram(&self, addr: u16) -> Result<u8, Error> {
        todo!()
    }

    fn write_ram(&self, addr: u16) -> Result<(), Error> {
        todo!()
    }
}

impl<A> FileOperation<A, Area> for Generic
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, addr: A) -> Result<u8, Error> {
        match addr.area_type() {
            Area::Rom => self.read_rom(u16::from(addr)),
            Area::Ram => self.read_ram(u16::from(addr)),
            _ => Err(Error::bus_error(u16::from(addr))),
        }
    }

    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        match addr.area_type() {
            Area::Rom => self.write_rom(u16::from(addr)),
            Area::Ram => self.write_ram(u16::from(addr)),
            _ => Err(Error::bus_error(u16::from(addr))),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GenericState {
    pub controller: Vec<u8>,
    pub ram: Option<Vec<u8>>,
}
