use gb_bus::{Address, Area, Error, FileOperation, Source};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

use crate::Header;

use super::save::StateError;
use super::{new_controller_from_header, Controller, Full, Partial};

pub struct Generic {
    controller: Box<dyn Controller>,
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
}

macro_rules! ram_op {
    ($controller:expr, $addr:expr, $ram:expr, $fn:expr) => {{
        let reladdr = $addr;
        if $controller.ram_enabled() {
            $ram.map_or_else(
                || Err(Error::new_segfault(reladdr)),
                |ram| {
                    let addr = $controller.offset_ram_addr(reladdr);
                    $fn(ram, addr)
                },
            )
        } else {
            Err(Error::new_segfault(reladdr))
        }
    }};
}

impl Generic {
    /// Create an empty Generic MBC from an header
    pub fn new(header: Header) -> Self {
        let ctl = new_controller_from_header(header);

        Self {
            rom: ctl.create_rom(),
            ram: ctl.create_ram(),
            controller: ctl,
        }
    }

    /// Create a Generic MBC from an header with is corresponding ROM data
    pub fn from_reader(header: Header, mut reader: impl Read) -> Result<Self, io::Error> {
        let mut mbc = Self::new(header);

        reader.read_exact(&mut mbc.rom)?;
        Ok(mbc)
    }

    pub fn save(&self) -> GenericState<Full> {
        GenericState {
            controller: self.controller.serialize(),
            ram: self.ram.clone(),
        }
    }

    pub fn save_partial(&self) -> GenericState<Partial> {
        GenericState {
            controller: self.controller.serialize_partial(),
            ram: self.ram.clone(),
        }
    }

    pub fn load(&mut self, state: GenericState<Full>) -> Result<(), StateError> {
        self.ram = state.ram;
        self.controller.load(state.controller)
    }

    pub fn load_partial(&mut self, state: GenericState<Partial>) -> Result<(), StateError> {
        self.ram = state.ram;
        self.controller.load_partial(state.controller)
    }

    fn read_rom(&self, addr: u16) -> Result<u8, Error> {
        let addr = self.controller.offset_rom_addr(addr);
        Ok(self.rom[addr])
    }

    fn write_rom(&mut self, v: u8, addr: u16) -> Result<(), Error> {
        #[cfg(feature = "debug_mbcs_register")]
        log::debug!("writing ROM({:04x}) <== {:02x}", addr, v);
        self.controller.write_rom(v, addr);
        Ok(())
    }

    fn read_ram(&self, reladdr: u16) -> Result<u8, Error> {
        let ctl = &self.controller;

        ctl.override_read_ram(reladdr).map_or_else(
            || {
                ram_op!(
                    ctl,
                    reladdr,
                    self.ram.as_ref(),
                    |ram: &[u8], addr: usize| Ok(ram[addr])
                )
            },
            Ok,
        )
    }

    fn write_ram(&mut self, v: u8, reladdr: u16) -> Result<(), Error> {
        let ctl = &mut self.controller;
        ctl.override_write_ram(v, reladdr).map_or_else(
            || {
                ram_op!(
                    ctl,
                    reladdr,
                    self.ram.as_mut(),
                    |ram: &mut [u8], addr: usize| {
                        ram[addr] = v;
                        Ok(())
                    }
                )
            },
            Ok,
        )
    }
}

impl<A> FileOperation<A, Area> for Generic
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        match addr.area_type() {
            Area::Rom => self.read_rom(u16::from(addr)),
            Area::ExtRam => self.read_ram(u16::from(addr)),
            _ => Err(Error::bus_error(u16::from(addr))),
        }
    }

    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        match addr.area_type() {
            Area::Rom => self.write_rom(v, u16::from(addr)),
            Area::ExtRam => self.write_ram(v, u16::from(addr)),
            _ => Err(Error::bus_error(u16::from(addr))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericState<CTL> {
    pub controller: CTL,
    pub ram: Option<Vec<u8>>,
}
