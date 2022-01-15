use crate::Header;

use super::save::{Full as Complete, Partial as Incomplete, SaveState, StateError};
use super::{Controller, ROM_BANK_SIZE};

pub fn new_controller(header: Header) -> Box<Mbc2> {
    Box::new(Mbc2 {
        rom_banks: header.rom_size.get_bank_amounts(),
        ..Default::default()
    })
}

pub struct Mbc2 {
    rom_banks: usize,
    ram_enabled: bool,
    rom_bank: u8,
    ram: Box<[u8; Mbc2::RAM_SIZE]>,
}

impl Default for Mbc2 {
    fn default() -> Self {
        Self {
            rom_banks: 0,
            ram_enabled: false,
            rom_bank: 1,
            ram: Box::new([0; Mbc2::RAM_SIZE]),
        }
    }
}

impl Mbc2 {
    const RAM_SIZE: usize = 512;
}

impl Controller for Mbc2 {
    fn sizes(&self) -> (usize, Option<usize>) {
        (self.rom_banks * ROM_BANK_SIZE, None)
    }

    fn write_rom(&mut self, v: u8, addr: u16) {
        if matches!((addr >> 8) & 0xff, 0x00..=0x3f) {
            if addr & 0x100 == 0x100 {
                self.rom_bank = (v & 0xf).max(1);
                #[cfg(feature = "debug_mbcs_register")]
                log::debug!("rom_bank={}", self.rom_bank);
            } else {
                self.ram_enabled = v & 0xf == 0xa;
                #[cfg(feature = "debug_mbcs_register")]
                log::debug!("ram_enabled={}", self.ram_enabled);
            }
        }
    }

    fn ram_enabled(&self) -> bool {
        false
    }

    fn override_read_ram(&self, addr: u16) -> Option<u8> {
        if self.ram_enabled {
            Some(self.ram[(addr & 0x1ff) as usize] | 0xf0)
        } else {
            None
        }
    }

    fn override_write_ram(&mut self, v: u8, addr: u16) -> Option<()> {
        if self.ram_enabled {
            self.ram[(addr & 0x1ff) as usize] = v | 0xf0;
            Some(())
        } else {
            None
        }
    }

    fn offset_ram_addr(&self, _addr: u16) -> usize {
        0
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Full {
    partial: Partial,
    rom_bank: u8,
    ram_enabled: bool,
}

impl From<&Mbc2> for Full {
    fn from(ctl: &Mbc2) -> Self {
        Self {
            partial: Partial::from(ctl.ram.clone()),
            rom_bank: ctl.rom_bank,
            ram_enabled: ctl.ram_enabled,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Partial {
    ram: Vec<u8>,
}

impl From<Box<[u8; Mbc2::RAM_SIZE]>> for Partial {
    fn from(ram: Box<[u8; Mbc2::RAM_SIZE]>) -> Self {
        Self { ram: ram.to_vec() }
    }
}

impl SaveState for Mbc2 {
    fn serialize(&self) -> Complete {
        Complete::Mbc2(Full::from(self))
    }

    fn load(&mut self, state: Complete) -> Result<(), StateError> {
        if let Complete::Mbc2(state) = state {
            self.ram_enabled = state.ram_enabled;
            self.rom_bank = state.rom_bank;
            self.load_partial(Incomplete::Mbc2(state.partial))?;

            Ok(())
        } else {
            Err(StateError::WrongType {
                expected: "mbc2",
                got: state.id(),
            })
        }
    }

    fn serialize_partial(&self) -> Incomplete {
        Incomplete::Mbc2(Partial::from(self.ram.clone()))
    }

    fn load_partial(&mut self, state: Incomplete) -> Result<(), StateError> {
        if let Incomplete::Mbc2(state) = state {
            self.ram =
                Box::new(
                    state
                        .ram
                        .try_into()
                        .map_err(|arr: Vec<u8>| StateError::RamLength {
                            expected: Mbc2::RAM_SIZE,
                            got: arr.len(),
                        })?,
                );
            Ok(())
        } else {
            Err(StateError::WrongType {
                expected: "mbc2",
                got: state.id(),
            })
        }
    }
}
