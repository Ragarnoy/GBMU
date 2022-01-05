use crate::Header;

use super::Controller;

pub fn new_controller(header: Header) -> Box<Mbc1> {
    Box::new(Mbc1 {})
}

pub struct Mbc1 {}

impl Controller for Mbc1 {
    fn sizes(&self) -> (usize, Option<usize>) {
        todo!()
    }

    fn save_to_slice(&self) -> Vec<u8> {
        todo!()
    }

    fn load_from_slice(&mut self, slice: &[u8]) {
        todo!()
    }

    fn write_rom(&mut self, v: u8, addr: u16) {
        todo!()
    }

    fn override_read_ram(&self, addr: u16) -> Option<u8> {
        todo!()
    }

    fn override_write_ram(&mut self, v: u8, addr: u16) -> Option<()> {
        todo!()
    }

    fn offset_ram_addr(&self, addr: u16) -> usize {
        todo!()
    }

    fn offset_rom_addr(&self, addr: u16) -> usize {
        todo!()
    }
}
