use crate::header::size::{RamSize, RomSize};
use gb_cpu::{address_bus::Error, FileOperation, Position, RomOperation};
use std::io::{self, Read};

pub const MBC1_ROM_SIZE: usize = 0x4000;
pub const MBC1_MAX_ROM_BANK: usize = 0x80;
pub const MBC1_RAM_SIZE: usize = 0x2000;
pub const MBC1_MAX_RAM_BANK: usize = 0x4;

pub struct MBC1 {
    configuration: Configuration,
    rom_bank: Vec<[u8; MBC1_ROM_SIZE]>,
    ram_bank: Vec<[u8; MBC1_RAM_SIZE]>,
    regs: MBC1Reg,
}

impl MBC1 {
    pub fn new(ram_size: RamSize, rom_size: RomSize) -> Self {
        let ram_bank = ram_size.get_bank_amounts();
        let rom_bank = rom_size.get_bank_amounts();

        Self {
            configuration: Configuration::from_sizes(ram_size, rom_size),
            rom_bank: vec![[0_u8; MBC1_ROM_SIZE]; rom_bank],
            ram_bank: vec![[0_u8; MBC1_RAM_SIZE]; ram_bank],
            regs: MBC1Reg::default(),
        }
    }

    pub fn from_file(
        mut file: impl Read,
        ram_size: RamSize,
        rom_size: RomSize,
    ) -> Result<Self, io::Error> {
        let mut ctl = Self::new(ram_size, rom_size);

        for e in ctl.rom_bank.iter_mut() {
            file.read_exact(e)?;
        }
        Ok(ctl)
    }

    fn get_selected_rom(&self, root_bank: bool) -> &[u8; MBC1_ROM_SIZE] {
        let index = if root_bank {
            self.get_main_rom_index()
        } else {
            self.get_extra_rom_index()
        };

        &self.rom_bank[index]
    }

    /// Return the rom index for the area 0x0000-0x3fff
    fn get_main_rom_index(&self) -> usize {
        if self.regs.banking_mode == BankingMode::Simple
            || self.configuration != Configuration::LargeRom
        {
            0
        } else {
            self.get_rom_index_special()
        }
    }

    fn get_rom_index_special(&self) -> usize {
        ((self.regs.special & 3) << 5) as usize
    }

    /// Return the rom index for the area 0x4000-0x7fff
    fn get_extra_rom_index(&self) -> usize {
        self.get_rom_index_special() | (((self.regs.rom_number & 0x1f) | 1) as usize)
    }

    fn get_selected_ram_mut(&mut self) -> &mut [u8; MBC1_RAM_SIZE] {
        if self.regs.banking_mode == BankingMode::Simple
            || self.configuration != Configuration::LargeRam
        {
            &mut self.ram_bank[0]
        } else {
            &mut self.ram_bank[(self.regs.special & 0x3) as usize]
        }
    }

    fn get_selected_ram(&self) -> &[u8; MBC1_RAM_SIZE] {
        if self.regs.banking_mode == BankingMode::Simple
            || self.configuration != Configuration::LargeRam
        {
            &self.ram_bank[0]
        } else {
            &self.ram_bank[(self.regs.special & 0x3) as usize]
        }
    }
}

#[cfg(test)]
mod test_mbc1 {
    use super::{BankingMode, Configuration, MBC1Reg, RamSize, RomSize, MBC1};

    #[test]
    fn basic_card() {
        let mut ctl = MBC1::new(RamSize::KByte8, RomSize::KByte256);
        unimplemented!();
    }

    #[test]
    fn large_rom() {
        let mut ctl = MBC1::new(RamSize::KByte8, RomSize::MByte1);
        unimplemented!();
    }

    #[test]
    fn large_ram() {
        let mut ctl = MBC1::new(RamSize::KByte32, RomSize::KByte256);
        unimplemented!();
    }
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

#[derive(PartialEq, Eq)]
enum BankingMode {
    Simple,
    Advanced,
}

#[derive(Debug, PartialEq, Eq)]
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

impl Configuration {
    fn from_sizes(ram: RamSize, rom: RomSize) -> Self {
        if rom >= RomSize::MByte1 {
            Self::LargeRom
        } else if ram > RamSize::KByte8 {
            Self::LargeRam
        } else {
            Self::None
        }
    }
}

#[test]
fn test_conf_sizes() {
    assert_eq!(
        Configuration::from_sizes(RamSize::KByte8, RomSize::MByte1),
        Configuration::LargeRom
    );
    assert_eq!(
        Configuration::from_sizes(RamSize::KByte32, RomSize::KByte256),
        Configuration::LargeRam
    );
    assert_eq!(
        Configuration::from_sizes(RamSize::KByte8, RomSize::KByte512),
        Configuration::None
    )
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
        match addr.relative {
            0x0000..=0x1fff => self.regs.ram_enabled = (v & 0xf) == 0xa,
            0x2000..=0x3fff => self.regs.rom_number = (v & 0x1f) | 1,
            0x4000..=0x5fff => {
                self.regs.special = v & 0x3;
            }
            0x6000..=0x7fff => {
                self.regs.banking_mode = if (v & 1) == 1 {
                    BankingMode::Advanced
                } else {
                    BankingMode::Simple
                }
            }
            _ => return Err(Error::SegmentationFault(addr.absolute)),
        }
        Ok(())
    }

    fn read_rom(&self, addr: Position) -> Result<u8, Error> {
        let root_bank = addr.relative < 0x3fff;
        let rom = self.get_selected_rom(root_bank);

        if root_bank {
            Ok(rom[addr.relative as usize])
        } else {
            let addr = addr.relative - 0x4000;
            Ok(rom[addr as usize])
        }
    }
}

impl FileOperation for MBC1 {
    fn write(&mut self, v: u8, addr: Position) -> Result<(), Error> {
        if !self.regs.ram_enabled {
            return Err(Error::SegmentationFault(addr.absolute));
        }
        let ram = self.get_selected_ram_mut();
        ram[addr.relative as usize] = v;
        Ok(())
    }

    fn read(&self, addr: Position) -> Result<u8, Error> {
        if !self.regs.ram_enabled {
            return Err(Error::SegmentationFault(addr.absolute));
        }
        let ram = self.get_selected_ram();
        Ok(ram[addr.relative as usize])
    }
}
