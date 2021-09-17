use super::RegisterArray;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone, Copy)]
pub struct Scrolling {
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
}

impl Scrolling {
    pub fn new() -> Self {
        Scrolling {
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
        }
    }
}

impl From<[u8; 4]> for Scrolling {
    fn from(bytes: [u8; 4]) -> Scrolling {
        Scrolling {
            scy: bytes[0],
            scx: bytes[1],
            ly: bytes[2],
            lyc: bytes[3],
        }
    }
}

impl From<Scrolling> for [u8; 4] {
    fn from(register: Scrolling) -> [u8; 4] {
        [register.scy, register.scx, register.ly, register.lyc]
    }
}

impl Index<usize> for Scrolling {
    type Output = u8;

    fn index(&self, id: usize) -> &Self::Output {
        match id {
            0 => &self.scy,
            1 => &self.scx,
            2 => &self.ly,
            3 => &self.lyc,
            _ => panic!("Out of bound index for scrolling register"),
        }
    }
}

impl IndexMut<usize> for Scrolling {
    fn index_mut(&mut self, id: usize) -> &mut Self::Output {
        match id {
            0 => &mut self.scy,
            1 => &mut self.scx,
            2 => &mut self.ly,
            3 => &mut self.lyc,
            _ => panic!("Out of bound index for scrolling register"),
        }
    }
}

impl RegisterArray<u8, 4> for Scrolling {
    const WRITE_POS: [bool; 4] = [true, true, false, true];
}
