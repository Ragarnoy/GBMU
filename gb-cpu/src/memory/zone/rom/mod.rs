mod controllers;
mod mbc;

use std::fs::File;
use std::io::prelude::*;

use controllers::romonly::*;
use mbc::*;

#[derive(Debug)]
pub struct Rom {
    mbc: Mbc,
    data: Vec<u8>,
}

impl Rom {
    pub fn new(bios: File, cartrige: File) -> Self {
        let mut data: Vec<u8> = Vec::new();
        data = bios.bytes().map(|x| x.unwrap()).collect();

        let mut rom: Vec<u8> = cartrige.bytes().map(|x| x.unwrap()).collect();
        data.append(&mut rom);

        Rom {
            mbc: Mbc::RomOnly,
            data,
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        match self.mbc {
            Mbc::RomOnly => RomOnly::read(&self.data, address)
        }
    }

    pub fn write(&mut self, address: usize, data: u8) {
        match self.mbc {
            Mbc::RomOnly => RomOnly::write(&mut self.data, address, data)
        }
    }
}
