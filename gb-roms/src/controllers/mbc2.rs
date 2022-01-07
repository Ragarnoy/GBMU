use crate::Header;

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

    fn save_to_slice(&self) -> Vec<u8> {
        let mut res = vec![self.ram_enabled as u8];
        res.extend(self.ram.iter());
        res
    }

    fn load_from_slice(&mut self, slice: &[u8]) {
        self.ram_enabled = slice[0] != 0;
        if let Ok(ram) = <[u8; Mbc2::RAM_SIZE]>::try_from(&slice[1..]) {
            self.ram = Box::new(ram);
        }
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
