use super::{Controller, MbcState, RAM_BANK_SIZE, ROM_BANK_SIZE};
use crate::header::{
    size::{RamSize, RomSize},
    Header,
};
use gb_bus::{Address, Area, Error, FileOperation};
use std::io::{self, Read};

pub struct MBC1 {
    configuration: Configuration,
    rom_banks: Vec<[u8; ROM_BANK_SIZE]>,
    ram_banks: Vec<[u8; RAM_BANK_SIZE]>,
    regs: MBC1Reg,
}

impl MBC1 {
    pub const MAX_ROM_BANK: usize = 0x80;
    pub const MAX_RAM_BANK: usize = 0x4;

    pub fn new(header: Header) -> Self {
        let ram_banks = header.ram_size.get_bank_amounts();
        let rom_banks = header.rom_size.get_bank_amounts();

        Self {
            configuration: Configuration::from_sizes(header.ram_size, header.rom_size),
            rom_banks: vec![[0_u8; ROM_BANK_SIZE]; rom_banks],
            ram_banks: vec![[0_u8; RAM_BANK_SIZE]; ram_banks],
            regs: MBC1Reg::default(),
        }
    }

    pub fn from_file(mut file: impl Read, header: Header) -> Result<Self, io::Error> {
        let mut ctl = Self::new(header);

        for e in ctl.rom_banks.iter_mut() {
            file.read_exact(e)?;
        }
        Ok(ctl)
    }

    pub fn with_state(mut self, state: Mbc1State) -> Result<Self, String> {
        self.ram_banks = state
            .ram_banks
            .into_iter()
            .map(<[u8; RAM_BANK_SIZE]>::try_from)
            .collect::<Result<Vec<[u8; RAM_BANK_SIZE]>, Vec<u8>>>()
            .map_err(|faulty| {
                &format!(
                    "invalid state banks size, expected {}, got {}",
                    RAM_BANK_SIZE,
                    faulty.len()
                )
            })?;

        Ok(self)
    }

    pub fn get_state(&self) -> Mbc1State {
        Mbc1State::from(self.ram_banks.clone())
    }

    fn write_rom(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        match addr.get_address() {
            0x0000..=0x1fff => self.regs.ram_enabled = (v & 0xf) == 0xa,
            0x2000..=0x3fff => {
                let index = v & 0x1f;
                self.regs.rom_number = if index == 0 { 1 } else { index };
            }
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
            _ => return Err(Error::new_segfault(addr)),
        }
        Ok(())
    }

    fn read_rom(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        let address = addr.get_address();
        let is_root_bank = address < 0x4000;
        let rom = self.get_selected_rom(is_root_bank);

        if is_root_bank {
            Ok(rom[address])
        } else {
            let address = address - 0x4000;
            Ok(rom[address])
        }
    }

    fn get_selected_rom(&self, root_bank: bool) -> &[u8; ROM_BANK_SIZE] {
        let index = if root_bank {
            self.get_main_rom_index()
        } else {
            self.get_extra_rom_index()
        };

        &self.rom_banks[index]
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
        let upper_index = self.get_rom_index_special();
        let index = self.regs.rom_number & 0x1f;

        if index == 0 {
            upper_index | 1
        } else {
            upper_index | index as usize
        }
    }

    fn write_ram(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        if !self.regs.ram_enabled {
            return Err(Error::new_segfault(addr));
        }
        let ram = self.get_selected_ram_mut();
        let address = addr.get_address();
        ram[address] = v;
        Ok(())
    }

    fn read_ram(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        if !self.regs.ram_enabled {
            return Err(Error::new_segfault(addr));
        }
        let ram = self.get_selected_ram();
        let address = addr.get_address();
        Ok(ram[address])
    }

    fn get_selected_ram_mut(&mut self) -> &mut [u8; RAM_BANK_SIZE] {
        let index = self.get_ram_index();

        &mut self.ram_banks[index]
    }

    fn get_selected_ram(&self) -> &[u8; RAM_BANK_SIZE] {
        &self.ram_banks[self.get_ram_index()]
    }

    fn get_ram_index(&self) -> usize {
        if self.regs.banking_mode == BankingMode::Simple
            || self.configuration != Configuration::LargeRam
        {
            0
        } else {
            (self.regs.special & 0x3) as usize
        }
    }
}

impl FileOperation<Area> for MBC1 {
    fn write(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        match addr.area_type() {
            Area::Rom => self.write_rom(v, addr),
            Area::Ram => self.write_ram(v, addr),
            _ => Err(Error::bus_error(addr)),
        }
    }

    fn read(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        match addr.area_type() {
            Area::Rom => self.read_rom(addr),
            Area::Ram => self.read_ram(addr),
            _ => Err(Error::bus_error(addr)),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Mbc1State {
    ram_banks: Vec<Vec<u8>>,
}

impl From<Vec<[u8; RAM_BANK_SIZE]>> for Mbc1State {
    fn from(banks: Vec<[u8; RAM_BANK_SIZE]>) -> Self {
        Self {
            ram_banks: banks.iter().map(|bank| bank.to_vec()).collect(),
        }
    }
}

impl Controller for MBC1 {
    fn save(&self) -> MbcState {
        MbcState::Mbc1(self.get_state())
    }
}

#[cfg(test)]
mod test_mbc1 {
    use super::{BankingMode, Configuration, MBC1};
    use crate::header::{
        size::{RamSize, RomSize},
        Header,
    };
    use gb_bus::{address::Address, Area};

    #[test]
    fn test_extra_rom_default_selection() {
        let mut ctl = MBC1::new(Header {
            ram_size: RamSize::KByte8,
            rom_size: RomSize::KByte256,
            ..Default::default()
        });

        ctl.regs.rom_number = 0;
        assert_eq!(ctl.get_extra_rom_index(), 1);

        ctl.regs.rom_number = 2;
        assert_eq!(ctl.get_extra_rom_index(), 2);
    }

    #[test]
    fn basic_card() {
        let mut ctl = MBC1::new(Header {
            ram_size: RamSize::KByte8,
            rom_size: RomSize::KByte256,
            ..Default::default()
        });

        assert_eq!(ctl.configuration, Configuration::Basic);
        assert_eq!(ctl.ram_banks.len(), RamSize::KByte8.get_bank_amounts());
        assert_eq!(ctl.rom_banks.len(), RomSize::KByte256.get_bank_amounts());

        assert_eq!(ctl.get_main_rom_index(), 0);
        assert_eq!(ctl.get_extra_rom_index(), 1);
        assert_eq!(ctl.get_ram_index(), 0);

        ctl.regs.rom_number = 11;
        ctl.regs.special = 2;

        assert_eq!(ctl.get_main_rom_index(), 0);
        assert_eq!(ctl.get_extra_rom_index(), (2 << 5) | 11);
        assert_eq!(ctl.get_ram_index(), 0);

        ctl.regs.banking_mode = BankingMode::Advanced;

        assert_eq!(ctl.get_main_rom_index(), 0);
        assert_eq!(ctl.get_extra_rom_index(), (2 << 5) | 11);
        assert_eq!(ctl.get_ram_index(), 0);

        ctl.regs.rom_number = 1;
        ctl.regs.special = 0;
        ctl.regs.banking_mode = BankingMode::Simple;

        ctl.rom_banks[0][0x3fff] = 51;
        ctl.rom_banks[1][0] = 42;
        let b = ctl
            .read_rom(Box::new(Address::from_offset(Area::Rom, 0x3fff, 0)))
            .expect("failed to read");
        assert_eq!(b, 51);
        let b = ctl
            .read_rom(Box::new(Address::from_offset(Area::Rom, 0x4000, 0)))
            .expect("failed to read");
        assert_eq!(b, 42);
    }

    #[test]
    fn large_rom() {
        let mut ctl = MBC1::new(Header {
            ram_size: RamSize::KByte8,
            rom_size: RomSize::MByte1,
            ..Default::default()
        });

        assert_eq!(ctl.configuration, Configuration::LargeRom);
        assert_eq!(ctl.ram_banks.len(), RamSize::KByte8.get_bank_amounts());
        assert_eq!(ctl.rom_banks.len(), RomSize::MByte1.get_bank_amounts());

        assert_eq!(ctl.get_main_rom_index(), 0);
        assert_eq!(ctl.get_extra_rom_index(), 1);
        assert_eq!(ctl.get_ram_index(), 0);

        ctl.regs.rom_number = 11;
        ctl.regs.special = 3;

        assert_eq!(ctl.get_main_rom_index(), 0);
        assert_eq!(ctl.get_extra_rom_index(), (3 << 5) | 11);
        assert_eq!(ctl.get_ram_index(), 0);

        ctl.regs.banking_mode = BankingMode::Advanced;

        assert_eq!(ctl.get_main_rom_index(), 3 << 5);
        assert_eq!(ctl.get_extra_rom_index(), (3 << 5) | 11);
        assert_eq!(ctl.get_ram_index(), 0);
    }

    #[test]
    fn large_ram() {
        let mut ctl = MBC1::new(Header {
            ram_size: RamSize::KByte32,
            rom_size: RomSize::KByte256,
            ..Default::default()
        });

        assert_eq!(ctl.configuration, Configuration::LargeRam);
        assert_eq!(ctl.ram_banks.len(), RamSize::KByte32.get_bank_amounts());
        assert_eq!(ctl.rom_banks.len(), RomSize::KByte256.get_bank_amounts());

        assert_eq!(ctl.get_main_rom_index(), 0);
        assert_eq!(ctl.get_extra_rom_index(), 1);
        assert_eq!(ctl.get_ram_index(), 0);

        ctl.regs.rom_number = 11;
        ctl.regs.special = 3;

        assert_eq!(ctl.get_main_rom_index(), 0);
        assert_eq!(ctl.get_extra_rom_index(), (3 << 5) | 11);
        assert_eq!(ctl.get_ram_index(), 0);

        ctl.regs.banking_mode = BankingMode::Advanced;

        assert_eq!(ctl.get_main_rom_index(), 0);
        assert_eq!(ctl.get_extra_rom_index(), (3 << 5) | 11);
        assert_eq!(ctl.get_ram_index(), 3);
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
    Basic,
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
            Self::Basic
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
        Configuration::Basic
    )
}
