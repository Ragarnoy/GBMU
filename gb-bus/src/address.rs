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
    /// # use gb_bus::{Address, Area};
    /// let pos = Address::from_offset(Area::Bios, 0x42, 0x10);
    ///
    /// assert_eq!(pos.absolute, 0x42);
    /// assert_eq!(pos.relative, 0x32);
    /// ```
    pub fn from_offset(area: Area, addr: u16, offset: u16) -> Self {
        Self::new(area, addr - offset, addr)
    }
}
