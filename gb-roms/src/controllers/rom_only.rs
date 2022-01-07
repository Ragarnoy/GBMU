use crate::Header;

use super::Controller;

pub fn new_controller(header: Header) -> Box<RomOnly> {
    Box::new(RomOnly::from_header(header))
}

pub struct RomOnly {
    rom_size: usize,
}

impl RomOnly {
    fn from_header(header: Header) -> Self {
        Self {
            rom_size: header.rom_size.get_rom_size(),
        }
    }
}

impl Controller for RomOnly {
    fn sizes(&self) -> (usize, Option<usize>) {
        (self.rom_size, None)
    }

    fn save_to_slice(&self) -> Vec<u8> {
        Vec::new()
    }

    fn load_from_slice(&mut self, _slice: &[u8]) {}

    fn write_rom(&mut self, _v: u8, _addr: u16) {}

    fn override_read_ram(&self, _addr: u16) -> Option<u8> {
        None
    }

    fn override_write_ram(&mut self, _v: u8, _addr: u16) -> Option<()> {
        None
    }

    fn offset_ram_addr(&self, _addr: u16) -> usize {
        usize::MAX
    }

    fn offset_rom_addr(&self, addr: u16) -> usize {
        addr as usize
    }

    fn ram_enabled(&self) -> bool {
        false
    }
}
