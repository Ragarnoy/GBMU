pub mod iter;

use iter::Iter;

use crate::{
    constant::{
        ERAM_START, ERAM_STOP, EXT_RAM_START, EXT_RAM_STOP, HRAM_START, HRAM_STOP, IE_REG,
        IO_REG_START, IO_REG_STOP, OAM_START, OAM_STOP, RAM_START, RAM_STOP, ROM_START, ROM_STOP,
        VRAM_START, VRAM_STOP,
    },
    Addr, Area, Error, FileOperation, Source,
};

use std::{cell::RefCell, rc::Rc};

macro_rules! match_area {
    ($sub_macro:ident, $self:expr, $addr:expr $(,$args:expr)*) => {
        match $addr {
            ROM_START..=ROM_STOP => $sub_macro!(ROM_START, $self.rom, Rom, $addr $(,$args)*),
            VRAM_START..=VRAM_STOP => $sub_macro!(VRAM_START, $self.vram, Vram, $addr $(,$args)*),
            EXT_RAM_START..=EXT_RAM_STOP => {
                $sub_macro!(EXT_RAM_START, $self.ext_ram, ExtRam, $addr $(,$args)*)
            }
            RAM_START..=RAM_STOP => $sub_macro!(RAM_START, $self.ram, Ram, $addr $(,$args)*),
            ERAM_START..=ERAM_STOP => $sub_macro!(ERAM_START, $self.eram, ERam, $addr $(,$args)*),
            OAM_START..=OAM_STOP => $sub_macro!(OAM_START, $self.oam, Oam, $addr $(,$args)*),
            IO_REG_START..=IO_REG_STOP => $sub_macro!(IO_REG_START, $self.io_reg, IoReg, $addr $(,$args)*),
            HRAM_START..=HRAM_STOP => $sub_macro!(HRAM_START, $self.hram, HighRam, $addr $(,$args)*),
            IE_REG => $sub_macro!(IE_REG, $self.ie_reg, IEReg, $addr $(,$args)*),
            _ => Err(Error::BusError($addr)),
        }
    };
}

macro_rules! write_area {
    ($start:expr, $field:expr, $area_type:ident, $addr:expr, $value:expr, $source:expr) => {{
        #[cfg(feature = "trace_bus_write")]
        log::trace!(
            "writing at {:4x} the value {:2x} in area {:?}",
            $addr,
            $value,
            Area::$area_type
        );
        $field.borrow_mut().write(
            $value,
            Addr::from_offset(Area::$area_type, $addr, $start),
            $source,
        )
    }};
}

macro_rules! read_area {
    ($start:expr, $field:expr, $area_type:ident, $addr: expr, $source:expr) => {{
        #[cfg(feature = "trace_bus_read")]
        log::trace!("reading at {:4x} in area {:?}", $addr, Area::$area_type);
        $field
            .borrow()
            .read(Addr::from_offset(Area::$area_type, $addr, $start), $source)
    }};
}

/// AddressBus map specific range address to specific area like ROM/RAM.
/// This Implementation of an AddressBus will be limited to 16-bit address
pub struct AddressBus {
    /// Rom from the cartridge
    pub rom: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
    /// Video Ram
    pub vram: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
    /// Ram from the cartridge
    pub ext_ram: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
    /// Internal gameboy ram
    pub ram: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
    /// Echo Ram area, usually a mirror of ram
    pub eram: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
    /// Sprite attribute table
    pub oam: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
    /// io registers table
    pub io_reg: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
    /// high ram
    /// allow for faster access in gameboy
    pub hram: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
    /// register to enable/disable all interrupts
    pub ie_reg: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
}

impl AddressBus {
    pub fn write_byte(&mut self, addr: u16, v: u8, source: Option<Source>) -> Result<(), Error> {
        match_area!(write_area, self, addr, v, source)
    }

    pub fn read_byte(&self, addr: u16, source: Option<Source>) -> Result<u8, Error> {
        match_area!(read_area, self, addr, source)
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

impl crate::Bus<u8> for AddressBus {
    fn read(&self, address: u16, source: Option<Source>) -> Result<u8, Error> {
        self.read_byte(address, source)
    }

    fn write(&mut self, address: u16, data: u8, source: Option<Source>) -> Result<(), Error> {
        self.write_byte(address, data, source)
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

        assert_eq!(addr_bus.read_byte(0x10, None), Ok(1));
        assert_eq!(addr_bus.read_byte(0x8042, None), Ok(2));
        assert_eq!(addr_bus.read_byte(0xa000, None), Ok(3));
        assert_eq!(addr_bus.read_byte(0xdfff, None), Ok(4));
        assert_eq!(addr_bus.read_byte(0xe000, None), Ok(5));
        assert_eq!(addr_bus.read_byte(0xfe00, None), Ok(6));
        assert_eq!(addr_bus.read_byte(0xff00, None), Ok(7));
        assert_eq!(addr_bus.read_byte(0xff80, None), Ok(8));
        assert_eq!(addr_bus.read_byte(0xffff, None), Ok(9));
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

        assert_eq!(addr_bus.write_byte(0x11, 0x30, None), Ok(()));
        assert_eq!(addr_bus.write_byte(0x8242, 0x31, None), Ok(()));
        assert_eq!(addr_bus.write_byte(0xa050, 0x32, None), Ok(()));
        assert_eq!(addr_bus.write_byte(0xdf8f, 0x33, None), Ok(()));
        assert_eq!(addr_bus.write_byte(0xe006, 0x34, None), Ok(()));
        assert_eq!(addr_bus.write_byte(0xfe80, 0x35, None), Ok(()));
        assert_eq!(addr_bus.write_byte(0xff70, 0x36, None), Ok(()));
        assert_eq!(addr_bus.write_byte(0xff8e, 0x37, None), Ok(()));
        assert_eq!(addr_bus.write_byte(0xffff, 0x38, None), Ok(()));

        assert_eq!(addr_bus.read_byte(0x10, None), Ok(0x30));
        assert_eq!(addr_bus.read_byte(0x8042, None), Ok(0x31));
        assert_eq!(addr_bus.read_byte(0xa000, None), Ok(0x32));
        assert_eq!(addr_bus.read_byte(0xdfff, None), Ok(0x33));
        assert_eq!(addr_bus.read_byte(0xe000, None), Ok(0x34));
        assert_eq!(addr_bus.read_byte(0xfe00, None), Ok(0x35));
        assert_eq!(addr_bus.read_byte(0xff00, None), Ok(0x36));
        assert_eq!(addr_bus.read_byte(0xff80, None), Ok(0x37));
        assert_eq!(addr_bus.read_byte(0xffff, None), Ok(0x38));
    }
}
