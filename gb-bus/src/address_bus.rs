use crate::file_operation::FileOperation;

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
    pub fn write(&mut self, v: u8, addr: u16) -> Result<(), Error> {
        match addr {
            0x0000..=0x00ff if self.bios.is_some() => {
                let b = self.bios.as_mut().unwrap();
                b.write(v, Address::from_offset(Area::Bios, addr, 0))
            }
            0x0000..=0x7fff => self.rom.write(v, Address::from_offset(Area::Rom, addr, 0)),
            0x8000..=0x9fff => self
                .vram
                .write(v, Address::from_offset(Area::Vram, addr, 0x8000)),
            0xa000..=0xbfff => self
                .ext_ram
                .write(v, Address::from_offset(Area::ExtRam, addr, 0xa000)),
            0xc000..=0xdfff => self
                .ram
                .write(v, Address::from_offset(Area::Ram, addr, 0xc000)),
            0xe000..=0xfdff => self
                .eram
                .write(v, Address::from_offset(Area::ERam, addr, 0xe000)),
            0xfe00..=0xfe9f => self
                .oam
                .write(v, Address::from_offset(Area::Oam, addr, 0xfe00)),
            0xff00..=0xff7f => self
                .io_reg
                .write(v, Address::from_offset(Area::IoReg, addr, 0xff00)),
            0xff80..=0xfffe => self
                .hram
                .write(v, Address::from_offset(Area::HighRam, addr, 0xff80)),
            0xffff => self
                .ie_reg
                .write(v, Address::from_offset(Area::IEReg, addr, 0xffff)),
            _ => Err(Error::BusError(Address::from_offset(
                Area::Unbound,
                addr,
                0,
            ))),
        }
    }

    pub fn read(&self, addr: u16) -> Result<u8, Error> {
        match addr {
            0x0000..=0x00ff if self.bios.is_some() => {
                let b = self.bios.as_ref().unwrap();
                b.read(Address::from_offset(Area::Bios, addr, 0))
            }
            0x0000..=0x7fff => self.rom.read(Address::from_offset(Area::Rom, addr, 0)),
            0x8000..=0x9fff => self
                .vram
                .read(Address::from_offset(Area::Vram, addr, 0x8000)),
            0xa000..=0xbfff => self
                .ext_ram
                .read(Address::from_offset(Area::ExtRam, addr, 0xa000)),
            0xc000..=0xdfff => self.ram.read(Address::from_offset(Area::Ram, addr, 0xc000)),
            0xe000..=0xfdff => self
                .eram
                .read(Address::from_offset(Area::ERam, addr, 0xe000)),
            0xfe00..=0xfe9f => self.oam.read(Address::from_offset(Area::Oam, addr, 0xfe00)),
            0xff00..=0xff7f => self
                .io_reg
                .read(Address::from_offset(Area::IoReg, addr, 0xff00)),
            0xff80..=0xfffe => self
                .hram
                .read(Address::from_offset(Area::HighRam, addr, 0xff80)),
            0xffff => self
                .ie_reg
                .read(Address::from_offset(Area::IEReg, addr, 0xffff)),
            _ => Err(Error::BusError(Address::from_offset(
                Area::Unbound,
                addr,
                0,
            ))),
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

        assert_eq!(addr_bus.read(0x10), Ok(1));
        assert_eq!(addr_bus.read(0x8042), Ok(2));
        assert_eq!(addr_bus.read(0xa000), Ok(3));
        assert_eq!(addr_bus.read(0xdfff), Ok(4));
        assert_eq!(addr_bus.read(0xe000), Ok(5));
        assert_eq!(addr_bus.read(0xfe00), Ok(6));
        assert_eq!(addr_bus.read(0xff00), Ok(7));
        assert_eq!(addr_bus.read(0xff80), Ok(8));
        assert_eq!(addr_bus.read(0xffff), Ok(9));
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

        assert_eq!(addr_bus.write(0x30, 0x11), Ok(()));
        assert_eq!(addr_bus.write(0x31, 0x8242), Ok(()));
        assert_eq!(addr_bus.write(0x32, 0xa050), Ok(()));
        assert_eq!(addr_bus.write(0x33, 0xdf8f), Ok(()));
        assert_eq!(addr_bus.write(0x34, 0xe006), Ok(()));
        assert_eq!(addr_bus.write(0x35, 0xfe80), Ok(()));
        assert_eq!(addr_bus.write(0x36, 0xff70), Ok(()));
        assert_eq!(addr_bus.write(0x37, 0xff8e), Ok(()));
        assert_eq!(addr_bus.write(0x38, 0xffff), Ok(()));

        assert_eq!(addr_bus.read(0x10), Ok(0x30));
        assert_eq!(addr_bus.read(0x8042), Ok(0x31));
        assert_eq!(addr_bus.read(0xa000), Ok(0x32));
        assert_eq!(addr_bus.read(0xdfff), Ok(0x33));
        assert_eq!(addr_bus.read(0xe000), Ok(0x34));
        assert_eq!(addr_bus.read(0xfe00), Ok(0x35));
        assert_eq!(addr_bus.read(0xff00), Ok(0x36));
        assert_eq!(addr_bus.read(0xff80), Ok(0x37));
        assert_eq!(addr_bus.read(0xffff), Ok(0x38));
    }
}

pub struct Iter<'a> {
    current_address: u16,
    stop: bool,
    bus: &'a AddressBus,
}

impl<'a> Iter<'a> {
    fn new(bus: &'a AddressBus) -> Self {
        Self {
            current_address: 0,
            stop: false,
            bus,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            let bit = self.bus.read(self.current_address).ok();
            match self.current_address {
                0xfea0..=0xfeff => self.current_address = 0xff00,
                0xffff => self.stop = true,
                _ => self.current_address += 1,
            }
            bit
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    BusError(Address),
    SegmentationFault(Address),
}

#[derive(Debug, PartialEq, Eq)]
/// Address contain the relative and absolute address
pub struct Address {
    /// relative address into the current area of the address bus
    pub relative: u16,

    /// absolute address used in the address bus
    pub absolute: u16,

    pub area: Area,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Area {
    Bios,
    Rom,
    Vram,
    ExtRam,
    Ram,
    ERam,
    Oam,
    IoReg,
    HighRam,
    IEReg,
    Unbound,
}

impl Address {
    pub fn new(area: Area, relative_addr: u16, absolute_addr: u16) -> Self {
        Self {
            relative: relative_addr,
            absolute: absolute_addr,
            area,
        }
    }

    /// Create an Address from an absolute adress and an offset
    ///
    /// ```
    /// # use gb_cpu::address_bus::{Address, Area};
    /// let pos = Address::from_offset(Area::Bios, 0x42, 0x10);
    ///
    /// assert_eq!(pos.absolute, 0x42);
    /// assert_eq!(pos.relative, 0x32);
    /// ```
    pub fn from_offset(area: Area, addr: u16, offset: u16) -> Self {
        Self::new(area, addr - offset, addr)
    }
}
