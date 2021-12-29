use super::{Controller, MbcStates, ROM_BANK_SIZE};
use crate::header::Header;
use gb_bus::{Address, Area, Error, FileOperation};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

pub struct MBC2 {
    rom_bank: Vec<[u8; ROM_BANK_SIZE]>,
    ram_bank: [u8; MBC2::RAM_SIZE],
    regs: MBC2Reg,
}

impl MBC2 {
    pub const MAX_ROM_BANK: usize = 0x10;
    pub const RAM_SIZE: usize = 0x200;

    fn new(header: Header) -> Self {
        let rom_banks_amount = header.rom_size.get_bank_amounts();

        Self {
            rom_bank: vec![[0_u8; ROM_BANK_SIZE]; rom_banks_amount],
            ram_bank: [0_u8; Self::RAM_SIZE],
            regs: MBC2Reg::default(),
        }
    }

    pub fn from_file(mut file: impl Read, header: Header) -> Result<Self, io::Error> {
        let mut ctl = Self::new(header);

        for e in ctl.rom_bank.iter_mut() {
            file.read_exact(e)?;
        }
        Ok(ctl)
    }

    pub fn with_state(&mut self, state: MbcState) -> Result<&Self, String> {
        self.ram_bank = <[u8; MBC2::RAM_SIZE]>::try_from(state.ram_bank).map_err(|faulty| {
            format!(
                "invalid state bank size, expected {}, got {}",
                MBC2::RAM_SIZE,
                faulty.len()
            )
        })?;
        Ok(self)
    }

    pub fn get_state(&self) -> MbcState {
        MbcState::from(self.ram_bank)
    }

    fn write_rom<A>(&mut self, v: u8, addr: A) -> Result<(), Error>
    where
        u16: From<A>,
        A: Address<Area>,
    {
        let address = addr.get_address();
        match address {
            0x0000..=0x3FFF if address & 0x100 == 0 => {
                self.regs.ram_enabled = v & 0xA == 0xA;
            }
            0x0000..=0x3FFF if address & 0x100 == 0x100 => {
                let v = v & 0xF;
                self.regs.rom_number = if v != 0 { v } else { 0x1 };
            }
            _ => return Err(Error::new_segfault(addr.into())),
        }
        Ok(())
    }

    fn read_rom<A>(&self, addr: A) -> Result<u8, Error>
    where
        u16: From<A>,
        A: Address<Area>,
    {
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

    fn get_selected_rom(&self, is_root_bank: bool) -> &[u8; ROM_BANK_SIZE] {
        let index: usize = if is_root_bank {
            0
        } else {
            self.regs.rom_number.into()
        };

        &self.rom_bank[index]
    }

    fn write_ram<A>(&mut self, v: u8, addr: A) -> Result<(), Error>
    where
        u16: From<A>,
        A: Address<Area>,
    {
        if !self.regs.ram_enabled {
            return Err(Error::new_segfault(addr.into()));
        }
        let address = addr.get_address();
        self.ram_bank[address] = v & 0xF;
        Ok(())
    }

    fn read_ram<A>(&self, addr: A) -> Result<u8, Error>
    where
        u16: From<A>,
        A: Address<Area>,
    {
        if !self.regs.ram_enabled {
            return Err(Error::new_segfault(addr.into()));
        }
        let address = addr.get_address();
        Ok(self.ram_bank[address])
    }
}

impl<A> FileOperation<A, Area> for MBC2
where
    u16: From<A>,
    A: Address<Area>,
{
    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        match addr.area_type() {
            Area::Rom => self.write_rom(v, addr),
            Area::Ram => self.write_ram(v, addr),
            _ => Err(Error::bus_error(addr.into())),
        }
    }

    fn read(&self, addr: A) -> Result<u8, Error> {
        match addr.area_type() {
            Area::Rom => self.read_rom(addr),
            Area::Ram => self.read_ram(addr),
            _ => Err(Error::bus_error(addr.into())),
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
#[derive(Serialize, Deserialize)]
pub struct MbcState {
    ram_bank: Vec<u8>,
}

impl From<[u8; MBC2::RAM_SIZE]> for MbcState {
    fn from(bank: [u8; MBC2::RAM_SIZE]) -> Self {
        Self {
            ram_bank: bank.to_vec(),
        }
    }
}

impl Controller for MBC2 {
    fn save(&self) -> MbcStates {
        MbcStates::Mbc2(self.get_state())
    }
}
