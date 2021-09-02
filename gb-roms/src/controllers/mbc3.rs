use crate::header::Header;
use std::cell::RefCell;
use std::io::{self, Read};

pub struct Mbc3 {
    rom_banks: Vec<[u8; Mbc3::ROM_BANK_SIZE]>,
    ram_banks: Vec<[u8; Mbc3::RAM_BANK_SIZE]>,
    regs: Mbc3Regs,
    clock_enabled: bool,
}

impl Mbc3 {
    pub const ROM_BANK_SIZE: usize = 0x4000;
    pub const RAM_BANK_SIZE: usize = 0x2000;

    pub fn from_reader(mut reader: impl Read, header: Header) -> Result<Self, io::Error> {
        let mut ctl = Mbc3::empty(header);

        Ok(ctl)
    }

    pub fn empty(header: Header) -> Self {
        use crate::header::cartridge_type::CartridgeType::{
            Mbc3TimerBattery, Mbc3TimerRamBattery2,
        };

        let ram_amount = header.ram_size.get_bank_amounts();
        let rom_amount = header.rom_size.get_bank_amounts();
        let clock_enabled = match header.cartridge_type {
            Mbc3TimerBattery | Mbc3TimerRamBattery2 => true,
            _ => false,
        };

        Self {
            ram_banks: vec![[0_u8; Mbc3::RAM_BANK_SIZE]; ram_amount],
            rom_banks: vec![[0_u8; Mbc3::ROM_BANK_SIZE]; rom_amount],
            regs: Mbc3Regs::default(),
            clock_enabled,
        }
    }
}

struct Mbc3Regs {
    rom_bank: usize,
    ram_bank: usize,
    last_writed_bytes: RefCell<u8>,
}

impl Default for Mbc3Regs {
    fn default() -> Self {
        Self {
            rom_bank: 0,
            ram_bank: 0,
            last_writed_bytes: RefCell::new(0),
        }
    }
}
