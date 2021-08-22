pub mod error;
pub mod iter;

pub use error::Error;
use iter::Iter;

use crate::FileOperation;
use crate::{Address, Area};

/// AddressBus map specific range address to specific area like ROM/RAM.
/// This Implementation of an AddressBus will be limited to 16-bit address
pub struct AddressBus {
    /// Optional BIOS Rom
    /// Usually set at startup then removed
    bios: Option<Box<dyn FileOperation>>,
    /// Rom from the cartridge
    rom: Box<dyn FileOperation>,
    /// Video Ram
    vram: Box<dyn FileOperation>,
    /// Ram from the cartridge
    ext_ram: Box<dyn FileOperation>,
    /// Internal gameboy ram
    ram: Box<dyn FileOperation>,
    /// Echo Ram area, usually a mirror of ram
    eram: Box<dyn FileOperation>,
    /// Sprite attribute table
    oam: Box<dyn FileOperation>,
    /// io registers table
    io_reg: Box<dyn FileOperation>,
    /// high ram
    /// allow for faster access in gameboy
    hram: Box<dyn FileOperation>,
    /// register to enable/disable all interrupts
    ie_reg: Box<dyn FileOperation>,
}

impl AddressBus {
    pub fn write_byte(&mut self, addr: u16, v: u8) -> Result<(), Error> {
        match addr {
            0x0000..=0x00ff if self.bios.is_some() => {
                let b = self.bios.as_mut().unwrap();
                b.write(v, Box::new(Address::from_offset(Area::Bios, addr, 0)))
            }
            0x0000..=0x7fff => self
                .rom
                .write(v, Box::new(Address::from_offset(Area::Rom, addr, 0))),
            0x8000..=0x9fff => self
                .vram
                .write(v, Box::new(Address::from_offset(Area::Vram, addr, 0x8000))),
            0xa000..=0xbfff => self.ext_ram.write(
                v,
                Box::new(Address::from_offset(Area::ExtRam, addr, 0xa000)),
            ),
            0xc000..=0xdfff => self
                .ram
                .write(v, Box::new(Address::from_offset(Area::Ram, addr, 0xc000))),
            0xe000..=0xfdff => self
                .eram
                .write(v, Box::new(Address::from_offset(Area::ERam, addr, 0xe000))),
            0xfe00..=0xfe9f => self
                .oam
                .write(v, Box::new(Address::from_offset(Area::Oam, addr, 0xfe00))),
            0xff00..=0xff7f => self
                .io_reg
                .write(v, Box::new(Address::from_offset(Area::IoReg, addr, 0xff00))),
            0xff80..=0xfffe => self.hram.write(
                v,
                Box::new(Address::from_offset(Area::HighRam, addr, 0xff80)),
            ),
            0xffff => self
                .ie_reg
                .write(v, Box::new(Address::from_offset(Area::IEReg, addr, 0xffff))),
            _ => Err(Error::BusError(Box::new(Address::from_offset(
                Area::Unbound,
                addr,
                0,
            )))),
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, Error> {
        match addr {
            0x0000..=0x00ff if self.bios.is_some() => {
                let b = self.bios.as_ref().unwrap();
                b.read(Box::new(Address::from_offset(Area::Bios, addr, 0)))
            }
            0x0000..=0x7fff => self
                .rom
                .read(Box::new(Address::from_offset(Area::Rom, addr, 0))),
            0x8000..=0x9fff => {
                self.vram
                    .read(Box::new(Address::from_offset(Area::Vram, addr, 0x8000)))
            }
            0xa000..=0xbfff => {
                self.ext_ram
                    .read(Box::new(Address::from_offset(Area::ExtRam, addr, 0xa000)))
            }
            0xc000..=0xdfff => {
                self.ram
                    .read(Box::new(Address::from_offset(Area::Ram, addr, 0xc000)))
            }
            0xe000..=0xfdff => {
                self.eram
                    .read(Box::new(Address::from_offset(Area::ERam, addr, 0xe000)))
            }
            0xfe00..=0xfe9f => {
                self.oam
                    .read(Box::new(Address::from_offset(Area::Oam, addr, 0xfe00)))
            }
            0xff00..=0xff7f => {
                self.io_reg
                    .read(Box::new(Address::from_offset(Area::IoReg, addr, 0xff00)))
            }
            0xff80..=0xfffe => {
                self.hram
                    .read(Box::new(Address::from_offset(Area::HighRam, addr, 0xff80)))
            }
            0xffff => self
                .ie_reg
                .read(Box::new(Address::from_offset(Area::IEReg, addr, 0xffff))),
            _ => Err(Error::BusError(Box::new(Address::from_offset(
                Area::Unbound,
                addr,
                0,
            )))),
        }
    }

    pub fn set_bios(&mut self, bios: Box<dyn FileOperation>) {
        self.bios = Some(bios)
    }

    pub fn remove_bios(&mut self) {
        self.bios = None
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

#[cfg(test)]
mod test_address_bus {
    use super::AddressBus;
    use crate::generic::CharDevice;

    #[test]
    fn read() {
        let addr_bus = AddressBus {
            bios: None,
            rom: Box::new(CharDevice(1)),
            vram: Box::new(CharDevice(2)),
            ext_ram: Box::new(CharDevice(3)),
            ram: Box::new(CharDevice(4)),
            eram: Box::new(CharDevice(5)),
            oam: Box::new(CharDevice(6)),
            io_reg: Box::new(CharDevice(7)),
            hram: Box::new(CharDevice(8)),
            ie_reg: Box::new(CharDevice(9)),
        };

        assert_eq!(addr_bus.read_byte(0x10), Ok(1));
        assert_eq!(addr_bus.read_byte(0x8042), Ok(2));
        assert_eq!(addr_bus.read_byte(0xa000), Ok(3));
        assert_eq!(addr_bus.read_byte(0xdfff), Ok(4));
        assert_eq!(addr_bus.read_byte(0xe000), Ok(5));
        assert_eq!(addr_bus.read_byte(0xfe00), Ok(6));
        assert_eq!(addr_bus.read_byte(0xff00), Ok(7));
        assert_eq!(addr_bus.read_byte(0xff80), Ok(8));
        assert_eq!(addr_bus.read_byte(0xffff), Ok(9));
    }

    #[test]
    fn write() {
        let mut addr_bus = AddressBus {
            bios: None,
            rom: Box::new(CharDevice(1)),
            vram: Box::new(CharDevice(2)),
            ext_ram: Box::new(CharDevice(3)),
            ram: Box::new(CharDevice(4)),
            eram: Box::new(CharDevice(5)),
            oam: Box::new(CharDevice(6)),
            io_reg: Box::new(CharDevice(7)),
            hram: Box::new(CharDevice(8)),
            ie_reg: Box::new(CharDevice(9)),
        };

        assert_eq!(addr_bus.write_byte(0x11, 0x30), Ok(()));
        assert_eq!(addr_bus.write_byte(0x8242, 0x31), Ok(()));
        assert_eq!(addr_bus.write_byte(0xa050, 0x32), Ok(()));
        assert_eq!(addr_bus.write_byte(0xdf8f, 0x33), Ok(()));
        assert_eq!(addr_bus.write_byte(0xe006, 0x34), Ok(()));
        assert_eq!(addr_bus.write_byte(0xfe80, 0x35), Ok(()));
        assert_eq!(addr_bus.write_byte(0xff70, 0x36), Ok(()));
        assert_eq!(addr_bus.write_byte(0xff8e, 0x37), Ok(()));
        assert_eq!(addr_bus.write_byte(0xffff, 0x38), Ok(()));

        assert_eq!(addr_bus.read_byte(0x10), Ok(0x30));
        assert_eq!(addr_bus.read_byte(0x8042), Ok(0x31));
        assert_eq!(addr_bus.read_byte(0xa000), Ok(0x32));
        assert_eq!(addr_bus.read_byte(0xdfff), Ok(0x33));
        assert_eq!(addr_bus.read_byte(0xe000), Ok(0x34));
        assert_eq!(addr_bus.read_byte(0xfe00), Ok(0x35));
        assert_eq!(addr_bus.read_byte(0xff00), Ok(0x36));
        assert_eq!(addr_bus.read_byte(0xff80), Ok(0x37));
        assert_eq!(addr_bus.read_byte(0xffff), Ok(0x38));
    }
}

impl crate::Bus<u8> for AddressBus {
    fn read(&self, address: u16) -> Result<u8, Error> {
        self.read_byte(address)
    }

    fn write(&mut self, address: u16, data: u8) -> Result<(), Error> {
        self.write_byte(address, data)
    }
}

impl crate::Bus<u16> for AddressBus {
    fn read(&self, address: u16) -> Result<u16, Error> {
        let lower = self.read_byte(address)?;
        let upper = self.read_byte(address + 1)?;

        Ok(u16::from_le_bytes([lower, upper]))
    }

    fn write(&mut self, address: u16, data: u16) -> Result<(), Error> {
        let [lower, upper] = data.to_le_bytes();

        self.write_byte(address, lower)?;
        self.write_byte(address + 1, upper)
    }
}
