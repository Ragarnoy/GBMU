use super::RegisterArray;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone, Copy)]
pub struct WindowPos {
    scy: u8,
    scx: u8,
}

impl WindowPos {
    pub fn new() -> Self {
        WindowPos { scy: 0, scx: 0 }
    }
}

impl From<[u8; 2]> for WindowPos {
    fn from(bytes: [u8; 2]) -> WindowPos {
        WindowPos {
            scy: bytes[0],
            scx: bytes[1],
        }
    }
}

impl From<WindowPos> for [u8; 2] {
    fn from(register: WindowPos) -> [u8; 2] {
        [register.scy, register.scx]
    }
}

impl Index<usize> for WindowPos {
    type Output = u8;

    fn index(&self, id: usize) -> &Self::Output {
        match id {
            0 => &self.scy,
            1 => &self.scx,
            _ => panic!("Out of bound index for WindowPos register"),
        }
    }
}

impl IndexMut<usize> for WindowPos {
    fn index_mut(&mut self, id: usize) -> &mut Self::Output {
        match id {
            0 => &mut self.scy,
            1 => &mut self.scx,
            _ => panic!("Out of bound index for WindowPos register"),
        }
    }
}

impl RegisterArray<u8, 2> for WindowPos {}
