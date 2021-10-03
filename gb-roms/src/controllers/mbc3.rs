use crate::header::Header;
use gb_bus::{Address, Area, Error};
use std::io::{self, Read};

type RamBank = [u8; MBC3::RAM_BANK_SIZE];
type RomBank = [u8; MBC3::ROM_BANK_SIZE];

pub struct MBC3 {
    rom_banks: Vec<RomBank>,
    ram_banks: Vec<RamBank>,
    regs: MBC3Regs,
    clock_enabled: bool,
}

impl MBC3 {
    pub const ROM_BANK_SIZE: usize = 0x4000;
    pub const RAM_BANK_SIZE: usize = 0x2000;

    pub fn from_reader(mut reader: impl Read, header: Header) -> Result<Self, io::Error> {
        let mut ctl = MBC3::empty(header);

        for e in ctl.rom_banks.iter_mut() {
            reader.read_exact(e)?;
        }
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
            ram_banks: vec![[0_u8; MBC3::RAM_BANK_SIZE]; ram_amount],
            rom_banks: vec![[0_u8; MBC3::ROM_BANK_SIZE]; rom_amount],
            regs: MBC3Regs::default(),
            clock_enabled,
        }
    }

    fn read_rom(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        let address = addr.get_address();
        match address {
            0x0000..=0x3FFF => Ok(self.rom_banks[0][address]),
            0x4000..=0x7FFF => Ok(self.get_selected_rom_bank()[address]),
            _ => Err(Error::new_segfault(addr)),
        }
    }

    fn get_selected_rom_bank(&self) -> &RomBank {
        &self.rom_banks[self.regs.rom_bank as usize]
    }

    fn write_rom(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        let address = addr.get_address();
        match address {
            0x0000..=0x1FFF => self.regs.ram_and_timer_enabled = (v & 0xF) == 0xA,
            0x2000..=0x3FFF => self.regs.rom_bank = if v == 0 { 1 } else { v & 0x7F },
            0x4000..=0x5FFF => self.regs.ram_bank = v & 0xC,
            0x6000..=0x7FFF => {
                if self.regs.last_writed_byte == Some(0_u8) && v == 1 {
                } else {
                    self.regs.last_writed_byte = Some(v);
                }
            }
            _ => return Err(Error::new_segfault(addr)),
        }
        Ok(())
    }
}

#[derive(Default)]
struct MBC3Regs {
    rom_bank: u8,
    ram_bank: u8,
    ram_and_timer_enabled: bool,
    rtc: RTCRegs,
    last_writed_byte: Option<u8>,
}

#[derive(Default)]
struct RTCRegs {
    seconds: u8,
    minutes: u8,
    hours: u8,
    lower_day_counter: u8,
    upper_day_counter: u8,
}
