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
        let mut ctl = MBC5::empty(ram_size, rom_size);

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
            regs: MBC5Reg::default(),
        }
    }

    fn write_rom(&mut self, v: u8, addr: Address) -> Result<(), Error> {
        match addr.relative {
            0x0000..=0x1FFF => self.regs.ram_enabled = (v & 0xf) == 0xa,
            0x2000..=0x2FFF => self.regs.set_lower_rom_number(v),
            0x3000..=0x3FFF => self.regs.set_upper_rom_number(v),
            0x4000..=0x5FFF => self.regs.set_ram_number(v),
            _ => return Err(Error::SegmentationFault(addr)),
        }
        Ok(())
    }

    fn read_rom(&self, addr: Address) -> Result<u8, Error> {
        match addr.relative {
            0x0000..=0x3FFF => Ok(self.rom_bank[0][addr.relative as usize]),
            0x4000..=0x7FFF => Ok(self.get_selected_rom()[addr.relative as usize]),
            _ => Err(Error::SegmentationFault(addr)),
        }
    }

    fn get_selected_rom(&self) -> &[u8; MBC5_ROM_BANK_SIZE] {
        &self.rom_bank[self.regs.rom_number as usize]
    }

    fn write_ram(&mut self, v: u8, addr: Address) -> Result<(), Error> {
        if !self.regs.ram_enabled {
            return Err(Error::SegmentationFault(addr));
        }
        let ram = self.get_selected_ram_mut();
        ram[addr.relative as usize] = v;
        Ok(())
    }

    fn read_ram(&self, addr: Address) -> Result<u8, Error> {
        if !self.regs.ram_enabled {
            return Err(Error::SegmentationFault(addr));
        }
        let ram = self.get_selected_ram();
        Ok(ram[addr.relative as usize])
    }

    fn get_selected_ram_mut(&mut self) -> &mut [u8; MBC5_RAM_BANK_SIZE] {
        &mut self.ram_bank[self.regs.ram_number as usize]
    }

    fn get_selected_ram(&self) -> &[u8; MBC5_RAM_BANK_SIZE] {
        &self.ram_bank[self.regs.ram_number as usize]
    }
}

impl FileOperation for MBC5 {
    fn read(&self, addr: Address) -> Result<u8, Error> {
        match addr.area {
            Area::Rom => self.read_rom(addr),
            Area::ExtRam => self.read_ram(addr),
            _ => panic!("mbc5 should not be mapped to the area {:?}", addr.area),
        }
    }

    fn write(&mut self, v: u8, addr: Address) -> Result<(), Error> {
        match addr.area {
            Area::Rom => self.write_rom(v, addr),
            Area::ExtRam => self.write_ram(v, addr),
            _ => panic!("mbc5 should not be mapped to the area {:?}", addr.area),
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

impl MBC5Reg {
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
            rom_number: 0,
            ram_number: 0,
        }
    }
}
