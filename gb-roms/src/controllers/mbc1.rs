use gb_cpu::{address_bus::Error, Position, RomOperation};

pub const MBC1_ROM_BANK_MAX_SIZE: usize = 0x4000;
pub const MBC1_MAX_ROM_BANK: usize = 0x80;
pub const MBC1_RAM_SIZE: usize = 0x2000;
pub const MBC1_MAX_RAM_BANK: usize = 0x4;

pub struct MBC1 {
    configuration: Configuration,
    regs: MBC1Reg,
}

struct MBC1Reg {
    /// Enable READ/WRITE operation on RAM
    ram_enabled: bool,
    /// Select ROM bank id in area 0x4000-0xbfff
    rom_number: u8,
    /// Special register that can be used to specify:
    /// - Rom Bank Number (0x[0246]0) on LargeROM on are 0x0000-0x3fff
    /// - Ram Bank Number on LargeRAM
    special: u8,
    /// This register has no effect when the controller is not in Large Ram/Rom
    banking_mode: BankingMode,
}

enum BankingMode {
    Simple,
    Advanced,
}

enum Configuration {
    /// When Card has one of:
    /// <= 8 KiB RAM
    /// <= 512 KiB ROM
    None,
    /// Rom mode when mbc1 has >= 1MiB
    LargeRom,
    /// Ram mode when mbc1 has > 8KiB RAM
    LargeRam,
}

impl Default for MBC1Reg {
    fn default() -> Self {
        Self {
            ram_enabled: false,
            rom_number: 1,
            special: 0,
            banking_mode: BankingMode::Simple,
        }
    }
}

impl RomOperation for MBC1 {
    fn write_rom(&mut self, v: u8, addr: Position) -> Result<(), Error> {
        Ok(())
    }

    fn read_rom(&self, addr: Position) -> Result<u8, Error> {
        unimplemented!("read operation are not implemented for mbc1 on rom")
    }
}
