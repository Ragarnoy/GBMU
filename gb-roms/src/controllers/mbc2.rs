use super::Controller;
use crate::header::size::RomSize;
use gb_bus::{Address, Area, Error, FileOperation};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

pub struct MBC2 {
    rom_bank: Vec<[u8; MBC2::ROM_SIZE]>,
    ram_bank: [u8; MBC2::RAM_SIZE],
    regs: MBC2Reg,
}

impl MBC2 {
    pub const ROM_SIZE: usize = 0x4000;
    pub const MAX_ROM_BANK: usize = 0x10;
    pub const RAM_SIZE: usize = 0x200;

    fn new(rom_size: RomSize) -> Self {
        let rom_banks_amount = rom_size.get_bank_amounts();

        Self {
            rom_bank: vec![[0_u8; Self::ROM_SIZE]; rom_banks_amount],
            ram_bank: [0_u8; Self::RAM_SIZE],
            regs: MBC2Reg::default(),
        }
    }

    pub fn from_file(mut file: impl Read, rom_size: RomSize) -> Result<Self, io::Error> {
        let mut ctl = Self::new(rom_size);

        for e in ctl.rom_bank.iter_mut() {
            file.read_exact(e)?;
        }
        Ok(ctl)
    }

    fn write_rom(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        let address = addr.get_address();
        match address {
            0x0000..=0x3FFF if address & 0x100 == 0 => {
                self.regs.ram_enabled = v & 0xA == 0xA;
            }
            0x0000..=0x3FFF if address & 0x100 == 0x100 => {
                let v = v & 0xF;
                self.regs.rom_number = if v != 0 { v } else { 0x1 };
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

    fn get_selected_rom(&self, is_root_bank: bool) -> &[u8; MBC2::ROM_SIZE] {
        let index: usize = if is_root_bank {
            0
        } else {
            self.regs.rom_number.into()
        };

        &self.rom_bank[index]
    }

    fn write_ram(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        if !self.regs.ram_enabled {
            return Err(Error::new_segfault(addr));
        }
        let address = addr.get_address();
        self.ram_bank[address] = v & 0xF;
        Ok(())
    }

    fn read_ram(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        if !self.regs.ram_enabled {
            return Err(Error::new_segfault(addr));
        }
        let address = addr.get_address();
        Ok(self.ram_bank[address])
    }
}

impl FileOperation<Area> for MBC2 {
    fn write(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        match addr.area_type() {
            Area::Rom => self.write_rom(v, addr),
            Area::Ram => self.write_ram(v, addr),
            _ => Err(Error::new_bus_error(addr)),
        }
    }

    fn read(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        match addr.area_type() {
            Area::Rom => self.read_rom(addr),
            Area::Ram => self.read_ram(addr),
            _ => Err(Error::new_bus_error(addr)),
        }
    }
}

struct MBC2Reg {
    /// Enable READ/WRITE operation on RAM
    ram_enabled: bool,
    /// Select ROM bank id in area 0x4000-0xbfff
    rom_number: u8,
}

impl Default for MBC2Reg {
    fn default() -> Self {
        Self {
            ram_enabled: false,
            rom_number: 1,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
struct Mbc2Data {
    ram_bank: Vec<u8>,
}

impl From<[u8; MBC2::RAM_SIZE]> for Mbc2Data {
    fn from(bank: [u8; MBC2::RAM_SIZE]) -> Self {
        Self {
            ram_bank: bank.to_vec(),
        }
    }
}

impl Controller for MBC2 {
    fn save<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data = Mbc2Data::from(self.ram_bank);
        data.serialize(serializer)
    }

    fn load<'de, D>(&mut self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        use std::convert::TryFrom;

        let data = Mbc2Data::deserialize(deserializer)?;

        self.ram_bank = <[u8; MBC2::RAM_SIZE]>::try_from(data.ram_bank).map_err(|faulty| {
            Error::invalid_length(
                faulty.len(),
                &format!("a ram bank size of size {}", MBC2::RAM_SIZE).as_str(),
            )
        })?;
        Ok(())
    }
}
