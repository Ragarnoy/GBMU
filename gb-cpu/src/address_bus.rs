/// AddressBus map specific range address to specific area like ROM/RAM.
/// This Implementation of an AddressBus will be limited to 16-bit address
pub struct AddressBus {
    bios: Option<Box<dyn RomOperation>>,
    rom: Box<dyn RomOperation>,
    vram: Box<dyn FileOperation>,
    ext_ram: Box<dyn FileOperation>,
    ram: Box<dyn FileOperation>,
    echo_ram: Box<dyn FileOperation>,
    sprite_table: Box<dyn FileOperation>,
    io_reg: Box<dyn FileOperation>,
    high_ram: Box<dyn FileOperation>,
    ie_reg: Box<dyn FileOperation>,
}

/// Position contain the relative and absolute address
pub struct Position {
    /// relative address is the relative address into the current area of the address bus
    pub relative: u16,

    /// absolute address is the absolute address used in the address bus
    pub absolute: u16,
}

/// RomOperation basic trait to implement for a ROM Emulator.
/// Rom is generally Read-only so `write` is not often used
pub trait RomOperation {
    /// writing to rom can be use full for MBC controller to set their own registry
    fn write(&mut self, _v: u8, _addr: Position) -> Result<(), ()> {
        Err(())
    }

    /// read one byte of data from rom
    fn read(&mut self, addr: Position) -> Result<u8, ()>;
}

/// FileOperation basic trait to implement for a RAM Emulator.
pub trait FileOperation {
    fn write(&mut self, v: u8, addr: Position) -> Result<(), ()>;
    fn read(&mut self, addr: Position) -> Result<u8, ()>;
}
