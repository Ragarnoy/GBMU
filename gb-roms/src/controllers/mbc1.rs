use crate::controllers::RAM_BANK_SIZE;
use crate::Header;

use super::{Controller, ROM_BANK_SIZE};

pub fn new_controller(header: Header) -> Box<Mbc1> {
    Box::new(Mbc1 {
        rom_banks: header.rom_size.get_bank_amounts(),
        ram_banks: header.ram_size.get_bank_amounts(),
        ..Default::default()
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
    bank_1: u8,
    /// Special register that depending on the mode will:
    /// - select the RAM bank
    /// - be the upper part of the ROM bank
    bank_2: u8,
    /// This register determine the behavior of the [special] register
    advance_mode: bool,
}

impl Default for Mbc1 {
    fn default() -> Self {
        Self {
            rom_banks: 0,
            ram_banks: 0,
            bank_1: 1,
            bank_2: 0,
            advance_mode: false,
            ram_enabled: false,
        }
    }
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
        // vec![
        //     self.ram_enabled as u8,
        //     self.bank_1,
        //     self.bank_2,
        //     self.advance_mode as u8,
        // ]
        Vec::new()
    }

    fn load_from_slice(&mut self, slice: &[u8]) {
        // self.ram_enabled = slice[0] != 0;
        // self.bank_1 = slice[1] & 0x1f;
        // self.bank_2 = slice[2] & 2;
        // self.advance_mode = slice[3] != 0;
    }

    fn write_rom(&mut self, v: u8, addr: u16) {
        #[cfg(feature = "debug_mbc1_register")]
        log::debug!("writing rom {:#02x} at {:#04x}", v, addr);
        match (addr >> 8) & 0xff {
            0x00..=0x1f => {
                self.ram_enabled = v & 0xf == 0xa;
                #[cfg(feature = "debug_mbc1_register")]
                log::debug!("ram_enabled={}", self.ram_enabled);
            }
            0x20..=0x3f => {
                self.bank_1 = (v & 0x1f).max(1);
                #[cfg(feature = "debug_mbc1_register")]
                log::debug!("bank_1={}", self.bank_1);
            }
            0x40..=0x5f => {
                self.bank_2 = v & 3;
                #[cfg(feature = "debug_mbc1_register")]
                log::debug!("bank_2={:x}", self.bank_2);
            }
            0x60..=0x7f => {
                self.advance_mode = v & 1 != 0;
                #[cfg(feature = "debug_mbc1_register")]
                log::debug!("advance_mode={}", self.advance_mode);
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
        let bank_number = raw_effective_ram_bank(self.bank_2, self.advance_mode);
        bank_number << 13 | (addr & 0x1fff) as usize
    }

    fn offset_rom_addr(&self, addr: u16) -> usize {
        let bank_number = raw_effective_rom_bank(self.bank_1, self.bank_2, self.advance_mode, addr);
        (bank_number % self.rom_banks) << 14 | (addr & 0x3fff) as usize
    }

    fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }
}

fn raw_effective_rom_bank(bank_1: u8, bank_2: u8, advance_mode: bool, addr: u16) -> usize {
    match addr >> 8 {
        0x00..=0x3f => {
            if advance_mode {
                (bank_2 << 5) as usize
            } else {
                0
            }
        }
        0x40..=0x7f => (bank_2 << 5 | bank_1) as usize,
        _ => panic!("unexpected addr to offset {:04x}", addr),
    }
}

fn raw_effective_ram_bank(bank_2: u8, mode: bool) -> usize {
    if mode {
        bank_2 as usize
    } else {
        0
    }
}

#[test]
fn t_raw_effective_rom_bank() {
    assert_eq!(raw_effective_rom_bank(0x12, 1, false, 0x4000), 0x32);
    assert_eq!(raw_effective_rom_bank(0x12, 1, false, 0x0000), 0);
    assert_eq!(raw_effective_rom_bank(0x12, 1, true, 0x0000), 0x20);
}

#[test]
fn offset_rom_addr() {
    let bank_1 = 4;
    let bank_2 = 2;
    let mbc = Mbc1 {
        rom_banks: usize::MAX,
        bank_1,
        bank_2,
        ..Default::default()
    };

    let addr = 0x72a7;
    let expect = 0x1132a7;
    assert_eq!(raw_effective_rom_bank(bank_1, bank_2, false, addr), 0x44);
    let res = mbc.offset_rom_addr(addr);
    assert_eq!(
        res, expect,
        "res = {0:x}({0:b}), expect = {1:x}({1:b})",
        res, expect
    );
}

#[test]
fn offset_ram_addr() {
    let bank_2 = 2;
    let mbc = Mbc1 {
        ram_banks: usize::MAX,
        bank_2,
        advance_mode: true,
        ..Default::default()
    };

    assert_eq!(raw_effective_ram_bank(bank_2, true), bank_2 as usize);
    let addr = 0xb123;
    let expect = 0x5123;
    let res = mbc.offset_ram_addr(addr);
    assert_eq!(
        res, expect,
        "res = {0:x}({0:b}), expect = {1:x}({1:b})",
        res, expect
    );
}
