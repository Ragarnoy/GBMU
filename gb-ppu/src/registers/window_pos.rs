use super::RegisterArray;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone, Copy)]
pub struct WindowPos {
    wy: u8,
    wx: u8,
}

impl WindowPos {
    pub fn new() -> Self {
        WindowPos { wy: 0, wx: 0 }
    }
}

impl From<[u8; 2]> for WindowPos {
    fn from(bytes: [u8; 2]) -> WindowPos {
        WindowPos {
            wy: bytes[0],
            wx: bytes[1],
        }
    }
}

impl From<WindowPos> for [u8; 2] {
    fn from(register: WindowPos) -> [u8; 2] {
        [register.wy, register.wx]
    }
}

impl Index<usize> for WindowPos {
    type Output = u8;

    fn index(&self, id: usize) -> &Self::Output {
        match id {
            0 => &self.wy,
            1 => &self.wx,
            _ => panic!("Out of bound index for WindowPos register"),
        }
    }
}

impl IndexMut<usize> for WindowPos {
    fn index_mut(&mut self, id: usize) -> &mut Self::Output {
        match id {
            0 => &mut self.wy,
            1 => &mut self.wx,
            _ => panic!("Out of bound index for WindowPos register"),
        }
    }
}

impl RegisterArray<u8, 2> for WindowPos {}
