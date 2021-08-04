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

    pub fn read(&mut self, addr: u16) -> Result<u8, Error> {
        match addr {
            0x0000..=0x00ff if self.bios.is_some() => {
                if let Some(ref mut b) = self.bios {
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
}

pub enum Error {
    BusError(u16),
    SegmentationFault(u16),
}

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

/// RomOperation basic trait to implement for a ROM Emulator.
/// Rom is generally Read-only so `write` is not often used
pub trait RomOperation {
    /// writing to rom can be use full for MBC controller to set their own registry
    fn write(&mut self, _v: u8, addr: Position) -> Result<(), Error> {
        Err(Error::SegmentationFault(addr.absolute))
    }

    /// read one byte of data from rom
    fn read(&mut self, addr: Position) -> Result<u8, Error>;
}

/// FileOperation basic trait to implement for a RAM Emulator.
pub trait FileOperation {
    fn write(&mut self, v: u8, addr: Position) -> Result<(), Error>;
    fn read(&mut self, addr: Position) -> Result<u8, Error>;
}
