use super::{Controller, MbcStates, RAM_BANK_SIZE, ROM_BANK_SIZE};
use crate::header::Header;
use gb_bus::{Address, Area, Error, FileOperation};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

pub struct MBC5 {
    rom_banks: Vec<[u8; ROM_BANK_SIZE]>,
    ram_banks: Vec<[u8; RAM_BANK_SIZE]>,
    regs: MBC5Reg,
}

impl MBC5 {
    pub const MAX_ROM_BANK: usize = 0x1FF;
    pub const MAX_RAM_BANK: usize = 0x10;

    /// empty return an empty initialized controller
    pub fn new(header: Header) -> MBC5 {
        let ram_bank = header.ram_size.get_bank_amounts();
        let rom_bank = header.rom_size.get_bank_amounts();

        Self {
            rom_banks: vec![[0_u8; ROM_BANK_SIZE]; rom_bank],
            ram_banks: vec![[0_u8; RAM_BANK_SIZE]; ram_bank],
            regs: MBC5Reg::default(),
        }
    }

    /// initialize the controller using a file as the rom
    pub fn from_file(mut file: impl Read, header: Header) -> Result<MBC5, io::Error> {
        let mut ctl = MBC5::new(header);

        for e in ctl.rom_banks.iter_mut() {
            file.read_exact(e)?;
        }
        Ok(ctl)
    }

    fn write_rom<A>(&mut self, v: u8, addr: A) -> Result<(), Error>
    where
        u16: From<A>,
        A: Address<Area>,
    {
        match addr.get_address() {
            0x0000..=0x1FFF => self.regs.set_ram_enabling_state(v),
            0x2000..=0x2FFF => self.regs.set_lower_rom_number(v),
            0x3000..=0x3FFF => self.regs.set_upper_rom_number(v),
            0x4000..=0x5FFF => self.regs.set_ram_number(v),
            _ => return Err(Error::new_segfault(addr.into())),
        }
        Ok(())
    }

    pub fn get_state(&self) -> MbcState {
        MbcState::from(self.ram_banks.clone())
    }

    pub fn with_state(&mut self, state: MbcState) -> Result<&Self, String> {
        if self.ram_banks.len() != state.ram_banks.len() {
            Err(format!(
                "invalid ram bank count, expected {}, got {}",
                self.ram_banks.len(),
                state.ram_banks.len()
            ))
        } else {
            self.ram_banks = state
                .ram_banks
                .into_iter()
                .map(<[u8; RAM_BANK_SIZE]>::try_from)
                .collect::<Result<Vec<[u8; RAM_BANK_SIZE]>, Vec<u8>>>()
                .map_err(|faulty| {
                    format!(
                        "invalid ram bank size, expected {}, got {}",
                        RAM_BANK_SIZE,
                        faulty.len()
                    )
                })?;
            Ok(self)
        }
    }

    fn read_rom<A>(&self, addr: A) -> Result<u8, Error>
    where
        u16: From<A>,
        A: Address<Area>,
    {
        let address = addr.get_address();
        match address {
            0x0000..=0x3FFF => Ok(self.rom_banks[0][address]),
            0x4000..=0x7FFF => Ok(self.get_selected_rom()[address - 0x4000]),
            _ => Err(Error::new_segfault(addr.into())),
        }
    }

    fn get_selected_rom(&self) -> &[u8; ROM_BANK_SIZE] {
        &self.rom_banks[self.regs.rom_number as usize]
    }

    fn write_ram<A>(&mut self, v: u8, addr: A) -> Result<(), Error>
    where
        u16: From<A>,
        A: Address<Area>,
    {
        if !self.regs.ram_enabled {
            return Err(Error::new_segfault(addr.into()));
        }
        let ram = self.get_selected_ram_mut();
        let address = addr.get_address();
        ram[address] = v;
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
        let ram = self.get_selected_ram();
        let address = addr.get_address();
        Ok(ram[address])
    }

    fn get_selected_ram_mut(&mut self) -> &mut [u8; RAM_BANK_SIZE] {
        &mut self.ram_banks[self.regs.ram_number as usize]
    }

    fn get_selected_ram(&self) -> &[u8; RAM_BANK_SIZE] {
        &self.ram_banks[self.regs.ram_number as usize]
    }
}

impl<A> FileOperation<A, Area> for MBC5
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, addr: A) -> Result<u8, Error> {
        match addr.area_type() {
            Area::Rom => self.read_rom(addr),
            Area::ExtRam => self.read_ram(addr),
            _ => Err(Error::bus_error(addr.into())),
        }
    }

    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        match addr.area_type() {
            Area::Rom => self.write_rom(v, addr),
            Area::ExtRam => self.write_ram(v, addr),
            _ => Err(Error::bus_error(addr.into())),
        }
    }
}

#[cfg(test)]
mod test_mbc5 {
    use super::MBC5;
    use crate::header::{
        size::{RamSize, RomSize},
        Header,
    };
    use gb_bus::{address::Addr, Area};

    #[test]
    fn basic() {
        let mut ctl = MBC5::new(Header {
            ram_size: RamSize::KByte32,
            rom_size: RomSize::KByte256,
            ..Default::default()
        });

        assert_eq!(ctl.ram_banks.len(), RamSize::KByte32.get_bank_amounts());
        assert_eq!(ctl.rom_banks.len(), RomSize::KByte256.get_bank_amounts());

        ctl.rom_banks[4][0x42] = 42;

        ctl.regs.set_lower_rom_number(4);
        assert_eq!(
            ctl.read_rom(Addr::from_offset(Area::Rom, 0x4042, 0)),
            Ok(42)
        );
        ctl.regs.set_ram_number(2);
        ctl.regs.set_ram_enabling_state(0xa);

        let addr = Addr::from_offset(Area::ExtRam, 0x42, 0);
        assert_eq!(ctl.write_ram(42, addr.clone()), Ok(()));
        assert_eq!(ctl.read_ram(addr), Ok(42));
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MBC5Reg {
    /// Enable read/write operation on the RAM
    ram_enabled: bool,
    /// Selected rom bank number
    rom_number: u16,
    /// Selected ram bank number
    ram_number: u8,
}

impl MBC5Reg {
    fn set_ram_enabling_state(&mut self, v: u8) {
        self.ram_enabled = (v & 0xf) == 0xa
    }

    fn set_lower_rom_number(&mut self, number: u8) {
        let upper = self.rom_number & 0x100;
        self.rom_number = upper | number as u16;
    }

    fn set_upper_rom_number(&mut self, number: u8) {
        let lower = self.rom_number & 0xff;
        self.rom_number = ((number & 1) as u16) << 8 | lower;
    }

    fn set_ram_number(&mut self, number: u8) {
        self.ram_number = number & 0xf
    }
}

impl Default for MBC5Reg {
    fn default() -> Self {
        Self {
            ram_enabled: false,
            rom_number: 1,
            ram_number: 0,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MbcState {
    ram_banks: Vec<Vec<u8>>,
}

impl std::convert::From<Vec<[u8; RAM_BANK_SIZE]>> for MbcState {
    fn from(ram_banks: Vec<[u8; RAM_BANK_SIZE]>) -> Self {
        Self {
            ram_banks: ram_banks.iter().map(|bank| bank.to_vec()).collect(),
        }
    }
}

impl Controller for MBC5 {
    fn save(&self) -> MbcStates {
        MbcStates::Mbc5(self.get_state())
    }
}

#[cfg(test)]
mod test_mbc5_regs {
    use super::MBC5Reg;

    #[test]
    fn default() {
        assert_eq!(
            MBC5Reg::default(),
            MBC5Reg {
                ram_enabled: false,
                rom_number: 1,
                ram_number: 0
            }
        )
    }

    #[test]
    fn ram_enabling() {
        let mut regs = MBC5Reg::default();

        assert!(!regs.ram_enabled);
        regs.set_ram_enabling_state(0xa);
        assert!(regs.ram_enabled);
        regs.set_ram_enabling_state(0);
        assert!(!regs.ram_enabled);
    }

    #[test]
    fn ram_number() {
        let mut regs = MBC5Reg::default();

        assert_eq!(regs.ram_number, 0);
        regs.set_ram_number(5);
        assert_eq!(regs.ram_number, 5);
    }

    #[test]
    fn rom_number() {
        let mut regs = MBC5Reg::default();

        assert_eq!(regs.rom_number, 1);
        regs.set_upper_rom_number(1);
        assert_eq!(regs.rom_number, 0x101);
        regs.set_lower_rom_number(0x42);
        assert_eq!(regs.rom_number, 0x142);
    }
}
