use super::RegisterArray;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub struct Scrolling {
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyc: u8,
}

impl Scrolling {
    pub const SIZE: usize = 4;
    pub const SCY: usize = 0;
    pub const SCX: usize = 1;
    pub const LY: usize = 2;
    pub const LYC: usize = 3;

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
            scy: bytes[Self::SCY],
            scx: bytes[Self::SCX],
            ly: bytes[Self::LY],
            lyc: bytes[Self::LYC],
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
            Self::SCY => &self.scy,
            Self::SCX => &self.scx,
            Self::LY => &self.ly,
            Self::LYC => &self.lyc,
            _ => panic!("Out of bound index for scrolling register"),
        }
    }
}

impl IndexMut<usize> for Scrolling {
    fn index_mut(&mut self, id: usize) -> &mut Self::Output {
        match id {
            Self::SCY => &mut self.scy,
            Self::SCX => &mut self.scx,
            Self::LY => &mut self.ly,
            Self::LYC => &mut self.lyc,
            _ => panic!("Out of bound index for scrolling register"),
        }
    }
}

impl RegisterArray<u8, 4> for Scrolling {
    const WRITE_POS: [bool; 4] = [true, true, false, true];
}
