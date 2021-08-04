pub mod operation;

pub use operation::{FileOperation, RomOperation};

/// AddressBus map specific range address to specific area like ROM/RAM.
/// This Implementation of an AddressBus will be limited to 16-bit address
pub struct AddressBus {
    /// Optional BIOS Rom
    /// Usually set at startup then removed
    bios: Option<Box<dyn RomOperation>>,
    /// Rom from the cartridge
    rom: Box<dyn RomOperation>,
    /// Video Ram
    vram: Box<dyn FileOperation>,
    /// Ram from the cartridge
    ext_ram: Box<dyn FileOperation>,
    /// Internal gameboy ram
    ram: Box<dyn FileOperation>,
    /// Echo Ram area, usually a mirron of ram
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
                if let Some(ref mut b) = self.bios {
                    b.write(v, Position::new(addr, addr))
                } else {
                    unreachable!("we already checked that bios is something")
                }
            }
            0x0000..=0x7fff => self.rom.write(v, Position::new(addr, addr)),
            0x8000..=0x9fff => self.vram.write(v, Position::from_offset(addr, 0x8000)),
            0xa000..=0xbfff => self.ext_ram.write(v, Position::from_offset(addr, 0xa000)),
            0xc000..=0xdfff => self.ram.write(v, Position::from_offset(addr, 0xc000)),
            0xe000..=0xfdff => self.eram.write(v, Position::from_offset(addr, 0xe000)),
            0xfe00..=0xfe9f => self.oam.write(v, Position::from_offset(addr, 0xfe00)),
            0xff00..=0xff7f => self.io_reg.write(v, Position::from_offset(addr, 0xff00)),
            0xff80..=0xfffe => self.hram.write(v, Position::from_offset(addr, 0xff80)),
            0xffff => self.ie_reg.write(v, Position::from_offset(addr, 0xffff)),
            _ => Err(Error::BusError(addr)),
        }
    }

    pub fn read(&self, addr: u16) -> Result<u8, Error> {
        match addr {
            0x0000..=0x00ff if self.bios.is_some() => {
                if let Some(ref b) = self.bios {
                    b.read(Position::new(addr, addr))
                } else {
                    unreachable!("we already checked that bios is something")
                }
            }
            0x0000..=0x7fff => self.rom.read(Position::new(addr, addr)),
            0x8000..=0x9fff => self.vram.read(Position::from_offset(addr, 0x8000)),
            0xa000..=0xbfff => self.ext_ram.read(Position::from_offset(addr, 0xa000)),
            0xc000..=0xdfff => self.ram.read(Position::from_offset(addr, 0xc000)),
            0xe000..=0xfdff => self.eram.read(Position::from_offset(addr, 0xe000)),
            0xfe00..=0xfe9f => self.oam.read(Position::from_offset(addr, 0xfe00)),
            0xff00..=0xff7f => self.io_reg.read(Position::from_offset(addr, 0xff00)),
            0xff80..=0xfffe => self.hram.read(Position::from_offset(addr, 0xff80)),
            0xffff => self.ie_reg.read(Position::from_offset(addr, 0xffff)),
            _ => Err(Error::BusError(addr)),
        }
    }

    pub fn set_bios(&mut self, bios: Box<dyn RomOperation>) {
        self.bios = Some(bios)
    }

    pub fn remove_bios(&mut self) {
        self.bios = None
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
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

#[derive(Debug)]
pub enum Error {
    BusError(u16),
    SegmentationFault(u16),
}

#[derive(Debug)]
/// Position contain the relative and absolute address
pub struct Position {
    /// relative address is the relative address into the current area of the address bus
    pub relative: u16,

    /// absolute address is the absolute address used in the address bus
    pub absolute: u16,
}

impl Position {
    pub fn new(relative_addr: u16, absolute_addr: u16) -> Self {
        Self {
            relative: relative_addr,
            absolute: absolute_addr,
        }
    }

    pub fn from_offset(addr: u16, offset: u16) -> Self {
        Self::new(addr - offset, addr)
    }
}
