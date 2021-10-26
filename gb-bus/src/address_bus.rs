pub mod iter;

use iter::Iter;

use crate::{
    address::Address,
    constant::{
        BIOS_START, BIOS_STOP, ERAM_START, ERAM_STOP, EXT_RAM_START, EXT_RAM_STOP, HRAM_START,
        HRAM_STOP, IE_REG_START, IO_REG_START, IO_REG_STOP, OAM_START, OAM_STOP, RAM_START,
        RAM_STOP, ROM_START, ROM_STOP, VRAM_START, VRAM_STOP,
    },
    Address as PseudoAddress, Area, Error, FileOperation, IORegArea,
};

use std::{cell::RefCell, rc::Rc};

/// AddressBus map specific range address to specific area like ROM/RAM.
/// This Implementation of an AddressBus will be limited to 16-bit address
pub struct AddressBus {
    /// Register to disable / enable the bios mapping.
    /// Set to non-zero to disable the bios mapping.
    bios_enabling_reg: u8,
    /// BIOS Rom
    bios: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Rom from the cartridge
    rom: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Video Ram
    vram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Ram from the cartridge
    ext_ram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Internal gameboy ram
    ram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Echo Ram area, usually a mirror of ram
    eram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Sprite attribute table
    oam: Rc<RefCell<dyn FileOperation<Area>>>,
    /// io registers table
    io_reg: Rc<RefCell<dyn FileOperation<Area>>>,
    /// high ram
    /// allow for faster access in gameboy
    hram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// register to enable/disable all interrupts
    ie_reg: Rc<RefCell<dyn FileOperation<Area>>>,
}

impl AddressBus {
    pub fn write_byte(&mut self, addr: u16, v: u8) -> Result<(), Error> {
        match addr {
            BIOS_START..=BIOS_STOP if self.bios_is_enabled() => self.bios.borrow_mut().write(
                v,
                Box::new(Address::from_offset(Area::Bios, addr, BIOS_START)),
            ),
            ROM_START..=ROM_STOP => self.rom.borrow_mut().write(
                v,
                Box::new(Address::from_offset(Area::Rom, addr, ROM_START)),
            ),
            VRAM_START..=VRAM_STOP => self.vram.borrow_mut().write(
                v,
                Box::new(Address::from_offset(Area::Vram, addr, VRAM_START)),
            ),
            EXT_RAM_START..=EXT_RAM_STOP => self.ext_ram.borrow_mut().write(
                v,
                Box::new(Address::from_offset(Area::ExtRam, addr, EXT_RAM_START)),
            ),
            RAM_START..=RAM_STOP => self.ram.borrow_mut().write(
                v,
                Box::new(Address::from_offset(Area::Ram, addr, RAM_START)),
            ),
            ERAM_START..=ERAM_STOP => self.eram.borrow_mut().write(
                v,
                Box::new(Address::from_offset(Area::ERam, addr, ERAM_START)),
            ),
            OAM_START..=OAM_STOP => self.oam.borrow_mut().write(
                v,
                Box::new(Address::from_offset(Area::Oam, addr, OAM_START)),
            ),
            IO_REG_START..=IO_REG_STOP => self.io_reg.borrow_mut().write(
                v,
                Box::new(Address::from_offset(Area::IoReg, addr, IO_REG_START)),
            ),
            HRAM_START..=HRAM_STOP => self.hram.borrow_mut().write(
                v,
                Box::new(Address::from_offset(Area::HighRam, addr, HRAM_START)),
            ),
            IE_REG_START => self
                .ie_reg
                .borrow_mut()
                .write(v, Box::new(Address::byte_reg(Area::IEReg, addr))),
            _ => Err(Error::BusError(addr)),
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, Error> {
        match addr {
            BIOS_START..=BIOS_STOP if self.bios_is_enabled() => self
                .bios
                .borrow()
                .read(Box::new(Address::from_offset(Area::Bios, addr, BIOS_START))),
            ROM_START..=ROM_STOP => {
                self.rom
                    .borrow()
                    .read(Box::new(Address::from_offset(Area::Rom, addr, ROM_START)))
            }
            VRAM_START..=VRAM_STOP => self.vram.borrow().read(Box::new(Address::from_offset(
                Area::Vram,
                addr,
                VRAM_START,
            ))),
            EXT_RAM_START..=EXT_RAM_STOP => self.ext_ram.borrow().read(Box::new(
                Address::from_offset(Area::ExtRam, addr, EXT_RAM_START),
            )),
            RAM_START..=RAM_STOP => {
                self.ram
                    .borrow()
                    .read(Box::new(Address::from_offset(Area::Ram, addr, RAM_START)))
            }
            ERAM_START..=ERAM_STOP => self.eram.borrow().read(Box::new(Address::from_offset(
                Area::ERam,
                addr,
                ERAM_START,
            ))),
            OAM_START..=OAM_STOP => {
                self.oam
                    .borrow()
                    .read(Box::new(Address::from_offset(Area::Oam, addr, OAM_START)))
            }
            IO_REG_START..=IO_REG_STOP => self.io_reg.borrow().read(Box::new(
                Address::from_offset(Area::IoReg, addr, IO_REG_START),
            )),
            HRAM_START..=HRAM_STOP => self.hram.borrow().read(Box::new(Address::from_offset(
                Area::HighRam,
                addr,
                HRAM_START,
            ))),
            IE_REG_START => self
                .ie_reg
                .borrow()
                .read(Box::new(Address::byte_reg(Area::IEReg, addr))),
            _ => Err(Error::BusError(addr)),
        }
    }

    pub fn bios_is_enabled(&self) -> bool {
        self.bios_enabling_reg == 0
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
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

impl FileOperation<IORegArea> for AddressBus {
    fn read(&self, address: Box<dyn PseudoAddress<IORegArea>>) -> Result<u8, Error> {
        let addr: u16 = address.into();
        if addr == 0 {
            Ok(self.bios_enabling_reg)
        } else {
            Err(Error::BusError(addr))
        }
    }

    fn write(&mut self, v: u8, address: Box<dyn PseudoAddress<IORegArea>>) -> Result<(), Error> {
        let addr: u16 = address.into();
        if addr == 0 {
            self.bios_enabling_reg = v;
            Ok(())
        } else {
            Err(Error::BusError(addr))
        }
    }
}

#[cfg(test)]
mod test_address_bus {
    use super::AddressBus;
    use crate::generic::CharDevice;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn read() {
        let addr_bus = AddressBus {
            bios_enabling_reg: 1,
            bios: Rc::new(RefCell::new(CharDevice(0))),
            rom: Rc::new(RefCell::new(CharDevice(1))),
            vram: Rc::new(RefCell::new(CharDevice(2))),
            ext_ram: Rc::new(RefCell::new(CharDevice(3))),
            ram: Rc::new(RefCell::new(CharDevice(4))),
            eram: Rc::new(RefCell::new(CharDevice(5))),
            oam: Rc::new(RefCell::new(CharDevice(6))),
            io_reg: Rc::new(RefCell::new(CharDevice(7))),
            hram: Rc::new(RefCell::new(CharDevice(8))),
            ie_reg: Rc::new(RefCell::new(CharDevice(9))),
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
            bios_enabling_reg: 1,
            bios: Rc::new(RefCell::new(CharDevice(0))),
            rom: Rc::new(RefCell::new(CharDevice(1))),
            vram: Rc::new(RefCell::new(CharDevice(2))),
            ext_ram: Rc::new(RefCell::new(CharDevice(3))),
            ram: Rc::new(RefCell::new(CharDevice(4))),
            eram: Rc::new(RefCell::new(CharDevice(5))),
            oam: Rc::new(RefCell::new(CharDevice(6))),
            io_reg: Rc::new(RefCell::new(CharDevice(7))),
            hram: Rc::new(RefCell::new(CharDevice(8))),
            ie_reg: Rc::new(RefCell::new(CharDevice(9))),
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
