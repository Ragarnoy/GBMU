use crate::controllers::RAM_BANK_SIZE;
use crate::Header;

use super::save::{Full as Complete, SaveState, StateError};
use super::{Controller, ROM_BANK_SIZE};

pub fn new_controller(header: Header) -> Box<Mbc5> {
    Box::new(Mbc5 {
        rom_banks: header.rom_size.get_bank_amounts(),
        ram_banks: header.ram_size.get_bank_amounts(),
        ..Default::default()
    })
}

pub struct Mbc5 {
    rom_banks: usize,
    ram_banks: usize,
    ram_enabled: bool,
    rom_bank: u16,
    ram_bank: u8,
}

impl Default for Mbc5 {
    fn default() -> Self {
        Self {
            rom_banks: 0,
            ram_banks: 0,
            ram_enabled: false,
            rom_bank: 1,
            ram_bank: 0,
        }
    }
}

impl Controller for Mbc5 {
    fn sizes(&self) -> (usize, Option<usize>) {
        (
            self.rom_banks * ROM_BANK_SIZE,
            if self.ram_banks > 0 {
                Some(self.ram_banks * RAM_BANK_SIZE)
            } else {
                None
            },
        )
    }

    fn write_rom(&mut self, v: u8, addr: u16) {
        match (addr >> 8) & 0xff {
            0x00..=0x1f => {
                self.ram_enabled = (v & 0xf) == 0xa;
                #[cfg(feature = "debug_mbcs_register")]
                log::debug!("ram_enabled={}", self.ram_enabled);
            }
            0x20..=0x2f => {
                self.rom_bank = (self.rom_bank & 0xff00) | v as u16;
                #[cfg(feature = "debug_mbcs_register")]
                log::debug!("rom_bank={} (low part)", self.rom_bank);
            }
            0x30..=0x3f => {
                self.rom_bank = self.rom_bank & 0xff | ((v & 1) as u16) << 8;
                #[cfg(feature = "debug_mbcs_register")]
                log::debug!("rom_bank={} (high part)", self.rom_bank);
            }
            0x40..=0x5f => {
                self.ram_bank = v & 0xf;
                #[cfg(feature = "debug_mbcs_register")]
                log::debug!("ram_bank={}", self.ram_bank);
            }
            _ => {}
        }
    }

    fn ram_enabled(&self) -> bool {
        self.ram_banks > 0 && self.ram_enabled
    }

    fn override_read_ram(&self, _addr: u16) -> Option<u8> {
        None
    }

    fn override_write_ram(&mut self, _v: u8, _addr: u16) -> Option<()> {
        None
    }

    fn offset_ram_addr(&self, addr: u16) -> usize {
        let bank = self.ram_bank as usize;
        ((bank % self.ram_banks) * RAM_BANK_SIZE) | (addr & 0x1fff) as usize
    }

    fn offset_rom_addr(&self, addr: u16) -> usize {
        let bank = if addr <= 0x3fff {
            0
        } else {
            self.rom_bank as usize
        };
        ((bank % self.rom_banks) * ROM_BANK_SIZE) | (addr & 0x3fff) as usize
    }
}

impl SaveState for Mbc5 {
    fn serialize(&self) -> Complete {
        Complete::Mbc5(Full::from(self))
    }

    fn load(&mut self, state: Complete) -> Result<(), StateError> {
        if let Complete::Mbc5(state) = state {
            self.ram_enabled = state.ram_enabled;
            self.rom_bank = state.rom_bank;
            self.ram_bank = state.ram_bank;

            Ok(())
        } else {
            Err(StateError::WrongType {
                expected: "mbc5",
                got: state.id(),
            })
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Full {
    ram_enabled: bool,
    rom_bank: u16,
    ram_bank: u8,
}

impl From<&Mbc5> for Full {
    fn from(ctl: &Mbc5) -> Self {
        Self {
            ram_enabled: ctl.ram_enabled,
            rom_bank: ctl.rom_bank,
            ram_bank: ctl.ram_bank,
        }
    }
}
