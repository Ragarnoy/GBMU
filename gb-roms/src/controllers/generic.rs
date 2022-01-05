use gb_bus::{Address, Area, Error, FileOperation};
use std::io::{self, Read};
use std::{cell::RefCell, rc::Rc};

use crate::header::CartridgeType;
use crate::{header, Header};

pub trait Controller {
    /// Return the size of the rom and optionnaly the size of the external ram
    fn sizes(&self) -> (usize, Option<usize>);
}

fn new_controller_from_header(header: Header) -> Rc<dyn Controller> {
    match header.cartridge_type {
        CartridgeType::Mbc1 => new_mbc1_controller(header),
        _ => panic!("unsupported cartridge type: {:?}", header.cartridge_type),
    }
}

pub struct Generic {
    controller: Rc<dyn Controller>,
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
}

impl Generic {
    fn new(header: Header) -> Self {
        let ctl = new_controller_from_header(header);
        let (rom_size, ram_size) = ctl.sizes();

        Self {
            controller: ctl,
            rom: vec![0; rom_size],
            ram: ram_size.map(|size| vec![0; size]),
        }
    }

    fn from_reader(header: Header, mut reader: impl Read) -> Result<Self, io::Error> {
        let mut mbc = Self::new(header);

        reader.read_exact(&mut mbc.rom)?;
        Ok(mbc)
    }
}

impl<A> FileOperation<A, Area> for Generic
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, addr: A) -> Result<u8, Error> {
        Ok(0xff)
    }

    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        Ok(())
    }
}
