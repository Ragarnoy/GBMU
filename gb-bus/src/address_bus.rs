pub mod iter;

use iter::Iter;

use crate::{
    address::Address,
    constant::{
        ERAM_START, ERAM_STOP, EXT_RAM_START, EXT_RAM_STOP, HRAM_START, HRAM_STOP, IE_REG_START,
        IO_REG_START, IO_REG_STOP, OAM_START, OAM_STOP, RAM_START, RAM_STOP, ROM_START, ROM_STOP,
        VRAM_START, VRAM_STOP,
    },
    Area, Error, FileOperation,
};

use std::{cell::RefCell, rc::Rc};

macro_rules! write_area {
    ($start:expr, $field:expr, $area_type:ident, $value:expr, $addr:expr) => {
        $field.borrow_mut().write(
            $value,
            Box::new(Address::from_offset(Area::$area_type, $addr, $start)),
        )
    };
}

macro_rules! read_area {
    ($start:expr, $field:expr, $area_type:ident, $addr: expr) => {
        $field.borrow().read(Box::new(Address::from_offset(
            Area::$area_type,
            $addr,
            $start,
        )))
    };
}

/// AddressBus map specific range address to specific area like ROM/RAM.
/// This Implementation of an AddressBus will be limited to 16-bit address
pub struct AddressBus {
    /// Rom from the cartridge
    pub rom: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Video Ram
    pub vram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Ram from the cartridge
    pub ext_ram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Internal gameboy ram
    pub ram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Echo Ram area, usually a mirror of ram
    pub eram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Sprite attribute table
    pub oam: Rc<RefCell<dyn FileOperation<Area>>>,
    /// io registers table
    pub io_reg: Rc<RefCell<dyn FileOperation<Area>>>,
    /// high ram
    /// allow for faster access in gameboy
    pub hram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// register to enable/disable all interrupts
    pub ie_reg: Rc<RefCell<dyn FileOperation<Area>>>,
}

impl AddressBus {
    pub fn write_byte(&mut self, addr: u16, v: u8) -> Result<(), Error> {
        match addr {
            ROM_START..=ROM_STOP => write_area!(ROM_START, self.rom, Rom, v, addr),
            VRAM_START..=VRAM_STOP => write_area!(VRAM_START, self.vram, Vram, v, addr),
            EXT_RAM_START..=EXT_RAM_STOP => {
                write_area!(EXT_RAM_START, self.ext_ram, ExtRam, v, addr)
            }
            RAM_START..=RAM_STOP => write_area!(RAM_START, self.ram, Ram, v, addr),
            ERAM_START..=ERAM_STOP => write_area!(ERAM_START, self.eram, ERam, v, addr),
            OAM_START..=OAM_STOP => write_area!(OAM_START, self.oam, Oam, v, addr),
            IO_REG_START..=IO_REG_STOP => write_area!(IO_REG_START, self.io_reg, IoReg, v, addr),
            HRAM_START..=HRAM_STOP => write_area!(HRAM_START, self.hram, HighRam, v, addr),
            IE_REG_START => write_area!(IE_REG_START, self.ie_reg, IEReg, v, addr),
            _ => Err(Error::BusError(addr)),
        }
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, Error> {
        match addr {
            ROM_START..=ROM_STOP => {
                read_area!(ROM_START, self.rom, Rom, addr)
            }
            VRAM_START..=VRAM_STOP => read_area!(VRAM_START, self.vram, Vram, addr),
            EXT_RAM_START..=EXT_RAM_STOP => read_area!(EXT_RAM_START, self.ext_ram, ExtRam, addr),
            RAM_START..=RAM_STOP => {
                read_area!(RAM_START, self.ram, Ram, addr)
            }
            ERAM_START..=ERAM_STOP => read_area!(ERAM_START, self.eram, ERam, addr),
            OAM_START..=OAM_STOP => {
                read_area!(OAM_START, self.oam, Oam, addr)
            }
            IO_REG_START..=IO_REG_STOP => read_area!(IO_REG_START, self.io_reg, IoReg, addr),
            HRAM_START..=HRAM_STOP => read_area!(HRAM_START, self.hram, HighRam, addr),
            IE_REG_START => read_area!(IE_REG_START, self.ie_reg, IEReg, addr),
            _ => Err(Error::BusError(addr)),
        }
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

#[cfg(test)]
mod test_address_bus {
    use super::AddressBus;
    use crate::generic::CharDevice;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn read() {
        let addr_bus = AddressBus {
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
