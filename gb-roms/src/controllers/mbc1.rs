use crate::controllers::RAM_BANK_SIZE;
use crate::Header;

use super::{Controller, ROM_BANK_SIZE};

pub fn new_controller(header: Header) -> Box<Mbc1> {
    Box::new(Mbc1 {
        rom_banks: header.rom_size.get_bank_amounts(),
        ram_banks: header.ram_size.get_bank_amounts(),
        ram_enabled: false,
        rom_bank_selected: 0,
        special: 0,
        advance_mode: false,
    })
}

pub struct Mbc1 {
    /// Number of ROM banks
    rom_banks: usize,
    /// Number of RAM banks
    ram_banks: usize,
    /// Register that enable to perform action on the RAM
    ram_enabled: bool,
    /// Lower register to select the BANK for the ROM
    rom_bank_selected: u8,
    /// Special register that depending on the mode will:
    /// - select the RAM bank
    /// - be the upper part of the ROM bank
    special: u8,
    /// This register determine the behavior of the [special] register
    advance_mode: bool,
}

impl Controller for Mbc1 {
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

    fn save_to_slice(&self) -> Vec<u8> {
        vec![
            self.ram_enabled as u8,
            self.rom_bank_selected,
            self.special,
            self.advance_mode as u8,
        ]
    }

    fn load_from_slice(&mut self, slice: &[u8]) {
        self.ram_enabled = slice[0] != 0;
        self.rom_bank_selected = slice[1] & 0x1f;
        self.special = slice[2] & 2;
        self.advance_mode = slice[3] != 0;
    }

    fn write_rom(&mut self, v: u8, addr: u16) {
        match (addr >> 8) & 0xff {
            0x00..=0x1f => {
                self.ram_enabled = v & 0xf == 0xa;
            }
            0x20..=0x3f => {
                self.rom_bank_selected = (v & 0x1f).max(1);
            }
            0x40..=0x5f => {
                self.special = v & 2;
            }
            0x60..=0x7f => {
                self.advance_mode = v & 1 != 0;
            }
            _ => {}
        }
    }

    fn override_read_ram(&self, _addr: u16) -> Option<u8> {
        None
    }

    fn override_write_ram(&mut self, _v: u8, _addr: u16) -> Option<()> {
        None
    }

    fn offset_ram_addr(&self, addr: u16) -> usize {
        todo!()
    }

    fn offset_rom_addr(&self, addr: u16) -> usize {
        let bank_number = match addr >> 8 {
            0x00..=0x3f => {
                if self.advance_mode {
                    (self.special << 5) as usize
                } else {
                    0
                }
            }
            0x40..=0x7f => (self.special << 5 | self.rom_bank_selected) as usize,
            _ => panic!("unexpected addr to offset {:04x}", addr),
        };
        (bank_number % self.rom_banks) * ROM_BANK_SIZE
    }

    fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }
}
