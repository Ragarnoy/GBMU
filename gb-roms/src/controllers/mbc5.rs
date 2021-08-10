use crate::header::size::{RamSize, RomSize};
use gb_cpu::address_bus::{Address, Area, Error, FileOperation};
use std::io::{self, Read};

pub const MBC5_ROM_BANK_SIZE: usize = 0x4000;
pub const MBC5_MAX_ROM_BANK: usize = 0x1FF;
pub const MBC5_RAM_BANK_SIZE: usize = 0x2000;
pub const MBC5_MAX_RAM_BANK: usize = 0x10;

pub struct MBC5 {
    rom_bank: Vec<[u8; MBC5_ROM_BANK_SIZE]>,
    ram_bank: Vec<[u8; MBC5_RAM_BANK_SIZE]>,
    regs: MBC5Reg,
}

impl MBC5 {
    /// initialize the controller using a file as the rom
    pub fn from_file(
        mut file: impl Read,
        ram_size: RamSize,
        rom_size: RomSize,
    ) -> Result<MBC5, io::Error> {
        let ctl = MBC5::empty(ram_size, rom_size);

        for e in ctl.rom_bank.iter_mut() {
            file.read_exact(e)?;
        }
        Ok(ctl)
    }

    /// empty return an empty initialized controller
    pub fn empty(ram_size: RamSize, rom_size: RomSize) -> MBC5 {
        let ram_bank = ram_size.get_bank_amounts();
        let rom_bank = rom_size.get_bank_amounts();

        Self {
            rom_bank: vec![[0_u8; MBC5_ROM_BANK_SIZE]; rom_bank],
            ram_bank: vec![[0_u8; MBC5_RAM_BANK_SIZE]; ram_bank],
        }
    }
}

impl FileOperation for MBC5 {
    fn read(&self, addr: Address) -> Result<u8, Error> {
        match addr.area {
            Area::Rom => unimplemented!("no read for mbc5 on ROM"),
            Area::ExtRam => unimplemented!("no read for mbc5 on RAM"),
            _ => panic!("mbc5 should not be mapped to the area {:?}", addr.area)
        }
    }

    fn write(&mut self, v: u8, addr: Address) -> Result<(), Error> {
        match addr.area {
            Area::Rom => unimplemented!("no write for mbc5 on ROM")
            Area::ExtRam => unimplemented!("no write for mbc5 on RAM")
            _ => panic!("mbc5 should not be mapped to the area {:?}", addr.area)
        }
    }
}

struct MBC5Reg {
    /// Enable read/write operation on the RAM
    ram_enabled: bool,
    /// Selected rom bank number
    rom_number: u16,
    /// Selected ram bank number
    ram_number: u8,
}

impl Default for MBC5Reg {
    fn default() -> Self {
        Self {
            ram_enabled: false,
            rom_number: 0,
            ram_number: 0,
        }
    }
}
