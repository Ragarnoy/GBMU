mod controllers;
mod mbc;

use std::io::Read;
use std::fs::File;

use controllers::romonly::*;
use mbc::*;
use crate::getset::*;

#[derive(Debug)]
pub struct Rom {
    mbc: Mbc,
    data: Vec<u8>,
}

impl Get<usize> for Rom {
    type Item = u8;

    fn get(&self, address: usize) -> u8 {
        match self.mbc {
            Mbc::RomOnly => RomOnly::read(&self.data, address)
        }
    }
}

impl Set<usize> for Rom {
    type Result = ();
    type Data = u8;

    fn set(&mut self, address: usize, data: u8) {
        match self.mbc {
            Mbc::RomOnly => RomOnly::write(&mut self.data, address, data)
        }
    }
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
}
