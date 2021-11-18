pub mod iter;

use iter::Iter;

use std::collections::HashMap;

use crate::{
    address::Address,
    constant::{
        ERAM_START, ERAM_STOP, EXT_RAM_START, EXT_RAM_STOP, HRAM_START, HRAM_STOP, IE_REG_START,
        IO_REG_START, IO_REG_STOP, OAM_START, OAM_STOP, RAM_START, RAM_STOP, ROM_START, ROM_STOP,
        UNDEFINED_VALUE, VRAM_START, VRAM_STOP,
    },
    Area, Error, FileOperation, InternalLock, Lock, MemoryLock,
};

use std::{cell::RefCell, rc::Rc};

macro_rules! write_area {
    ($start:expr, $field:expr, $area_type:ident, $value:expr, $addr:expr) => {{
        #[cfg(features = "trace_bus_write")]
        log::trace!(
            "writing at {:4x} the value {:2x} in area {:?}",
            $addr,
            $value,
            Area::$area_type
        );
        $field.borrow_mut().write(
            $value,
            Box::new(Address::from_offset(Area::$area_type, $addr, $start)),
        )
    }};
}

macro_rules! read_area {
    ($start:expr, $field:expr, $area_type:ident, $addr: expr) => {{
        #[cfg(features = "trace_bus_read")]
        log::trace!("reading at {:4x} in area {:?}", $addr, Area::$area_type);
        $field.borrow().read(Box::new(Address::from_offset(
            Area::$area_type,
            $addr,
            $start,
        )))
    }};
}

/// AddressBus map specific range address to specific area like ROM/RAM.
/// This Implementation of an AddressBus will be limited to 16-bit address
pub struct AddressBus {
    /// Rom from the cartridge
    pub rom: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Video Ram
    pub vram: Rc<RefCell<dyn InternalLock<Area>>>,
    /// Ram from the cartridge
    pub ext_ram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Internal gameboy ram
    pub ram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Echo Ram area, usually a mirror of ram
    pub eram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// Sprite attribute table
    pub oam: Rc<RefCell<dyn InternalLock<Area>>>,
    /// io registers table
    pub io_reg: Rc<RefCell<dyn FileOperation<Area>>>,
    /// high ram
    /// allow for faster access in gameboy
    pub hram: Rc<RefCell<dyn FileOperation<Area>>>,
    /// register to enable/disable all interrupts
    pub ie_reg: Rc<RefCell<dyn FileOperation<Area>>>,
    /// map a memory area to its current lock status
    pub area_locks: HashMap<Area, Lock>,
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

impl MemoryLock for AddressBus {
    fn lock(&mut self, area: Area, lock: Lock) {
        self.area_locks.insert(area, lock);
        match area {
            Area::Vram => self.vram.borrow_mut().lock(area, lock),
            Area::Oam => self.oam.borrow_mut().lock(area, lock),
            _ => {}
        }
    }

    fn unlock(&mut self, area: Area) {
        self.area_locks.remove(&area);
        match area {
            Area::Vram => self.vram.borrow_mut().unlock(area),
            Area::Oam => self.oam.borrow_mut().unlock(area),
            _ => {}
        }
    }

    fn is_available(&self, area: Area, lock_key: Option<Lock>) -> bool {
        if let Some(lock) = self.area_locks.get(&area) {
            if let Some(key) = lock_key {
                return *lock <= key;
            }
        } else {
            return true;
        }
        false
    }
}

impl crate::Bus<u8> for AddressBus {
    fn read(&self, address: u16, lock_key: Option<Lock>) -> Result<u8, Error> {
        if self.is_available(address.into(), lock_key) {
            self.read_byte(address)
        } else {
            Ok(UNDEFINED_VALUE)
        }
    }

    fn write(&mut self, address: u16, data: u8, lock_key: Option<Lock>) -> Result<(), Error> {
        if self.is_available(address.into(), lock_key) {
            self.write_byte(address, data)
        } else {
            Ok(())
        }
    }
}

impl crate::Bus<u16> for AddressBus {
    fn read(&self, address: u16, lock_key: Option<Lock>) -> Result<u16, Error> {
        if self.is_available(address.into(), lock_key) {
            let lower = self.read_byte(address)?;
            let upper = self.read_byte(address + 1)?;

            Ok(u16::from_le_bytes([lower, upper]))
        } else {
            Ok(u16::from_le_bytes([UNDEFINED_VALUE, UNDEFINED_VALUE]))
        }
    }

    fn write(&mut self, address: u16, data: u16, lock_key: Option<Lock>) -> Result<(), Error> {
        if self.is_available(address.into(), lock_key) {
            let [lower, upper] = data.to_le_bytes();

            self.write_byte(address, lower)?;
            self.write_byte(address + 1, upper)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test_address_bus {
    use super::AddressBus;
    use crate::generic::CharDevice;
    use std::collections::HashMap;
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
            area_locks: HashMap::new(),
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
            area_locks: HashMap::new(),
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

#[cfg(test)]
mod memory_locking {
    use super::{AddressBus, Area, Lock, MemoryLock};
    use crate::generic::CharDevice;
    use std::collections::HashMap;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn allow_stronger_key() {
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
            area_locks: HashMap::new(),
        };

        assert!(addr_bus.is_available(Area::Vram, None));
        assert!(addr_bus.is_available(Area::Vram, Some(Lock::Ppu)));
        assert!(addr_bus.is_available(Area::Vram, Some(Lock::Dma)));
        assert!(addr_bus.is_available(Area::Vram, Some(Lock::Debugger)));

        addr_bus.lock(Area::Vram, Lock::Ppu);
        assert!(!addr_bus.is_available(Area::Vram, None));
        assert!(addr_bus.is_available(Area::Vram, Some(Lock::Ppu)));
        assert!(addr_bus.is_available(Area::Vram, Some(Lock::Dma)));
        assert!(addr_bus.is_available(Area::Vram, Some(Lock::Debugger)));

        addr_bus.lock(Area::Vram, Lock::Dma);
        assert!(!addr_bus.is_available(Area::Vram, None));
        assert!(!addr_bus.is_available(Area::Vram, Some(Lock::Ppu)));
        assert!(addr_bus.is_available(Area::Vram, Some(Lock::Dma)));
        assert!(addr_bus.is_available(Area::Vram, Some(Lock::Debugger)));

        addr_bus.lock(Area::Vram, Lock::Debugger);
        assert!(!addr_bus.is_available(Area::Vram, None));
        assert!(!addr_bus.is_available(Area::Vram, Some(Lock::Ppu)));
        assert!(!addr_bus.is_available(Area::Vram, Some(Lock::Dma)));
        assert!(addr_bus.is_available(Area::Vram, Some(Lock::Debugger)));
    }

    #[test]
    fn lock_unlock() {
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
            area_locks: HashMap::new(),
        };

        assert!(addr_bus.is_available(Area::Vram, None));

        addr_bus.lock(Area::Vram, Lock::Ppu);
        assert!(!addr_bus.is_available(Area::Vram, None));

        addr_bus.unlock(Area::Vram);
        assert!(addr_bus.is_available(Area::Vram, None));
    }
}
