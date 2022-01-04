use super::RegisterArray;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone, Copy, Debug)]
pub struct WindowPos {
    pub wy: u8,
    pub wx: u8,
}

impl WindowPos {
    pub const SIZE: usize = 2;
    pub const WY: usize = 0;
    pub const WX: usize = 1;

    pub fn new() -> Self {
        WindowPos { wy: 0, wx: 0 }
    }
}

impl From<[u8; 2]> for WindowPos {
    fn from(bytes: [u8; 2]) -> WindowPos {
        WindowPos {
            wy: bytes[Self::WY],
            wx: bytes[Self::WX],
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
            Self::WY => &self.wy,
            Self::WX => &self.wx,
            _ => panic!("Out of bound index for WindowPos register"),
        }
    }
}

impl IndexMut<usize> for WindowPos {
    fn index_mut(&mut self, id: usize) -> &mut Self::Output {
        match id {
            Self::WY => &mut self.wy,
            Self::WX => &mut self.wx,
            _ => panic!("Out of bound index for WindowPos register"),
        }
    }
}

impl RegisterArray<u8, 2> for WindowPos {}
