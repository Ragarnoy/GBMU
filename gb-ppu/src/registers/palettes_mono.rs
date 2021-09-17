use super::RegisterArray;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone, Copy)]
pub struct PalettesMono {
    bg: u8,
    obj_0: u8,
    obj_1: u8,
}

impl PalettesMono {
    pub fn new() -> Self {
        PalettesMono {
            bg: 0,
            obj_0: 0,
            obj_1: 0,
        }
    }
}

impl From<[u8; 3]> for PalettesMono {
    fn from(bytes: [u8; 3]) -> PalettesMono {
        PalettesMono {
            bg: bytes[0],
            obj_0: bytes[1],
            obj_1: bytes[2],
        }
    }
}

impl From<PalettesMono> for [u8; 3] {
    fn from(register: PalettesMono) -> [u8; 3] {
        [register.bg, register.obj_0, register.obj_1]
    }
}

impl Index<usize> for PalettesMono {
    type Output = u8;

    fn index(&self, id: usize) -> &Self::Output {
        match id {
            0 => &self.bg,
            1 => &self.obj_0,
            2 => &self.obj_1,
            _ => panic!("Out of bound index for PaletteMono register"),
        }
    }
}

impl IndexMut<usize> for PalettesMono {
    fn index_mut(&mut self, id: usize) -> &mut Self::Output {
        match id {
            0 => &mut self.bg,
            1 => &mut self.obj_0,
            2 => &mut self.obj_1,
            _ => panic!("Out of bound index for PaletteMono register"),
        }
    }
}

impl RegisterArray<u8, 3> for PalettesMono {}
