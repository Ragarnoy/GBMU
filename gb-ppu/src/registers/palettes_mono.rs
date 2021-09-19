use super::{Palette, RegisterArray};
use std::ops::{Index, IndexMut};

#[derive(Default, Clone, Copy)]
pub struct PalettesMono {
    bg: Palette,
    obj_0: Palette,
    obj_1: Palette,
}

impl PalettesMono {
    pub fn new() -> Self {
        PalettesMono {
            bg: Palette::new(),
            obj_0: Palette::new(),
            obj_1: Palette::new(),
        }
    }

    pub fn bg(&self) -> &Palette {
        &self.bg
    }

    pub fn obj(&self) -> (&Palette, &Palette) {
        (&self.obj_0, &self.obj_1)
    }
}

impl From<[u8; 3]> for PalettesMono {
    fn from(bytes: [u8; 3]) -> PalettesMono {
        PalettesMono {
            bg: bytes[0].into(),
            obj_0: bytes[1].into(),
            obj_1: bytes[2].into(),
        }
    }
}

impl From<PalettesMono> for [u8; 3] {
    fn from(register: PalettesMono) -> [u8; 3] {
        [
            register.bg.into(),
            register.obj_0.into(),
            register.obj_1.into(),
        ]
    }
}

impl Index<usize> for PalettesMono {
    type Output = Palette;

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

impl RegisterArray<Palette, 3> for PalettesMono {}
