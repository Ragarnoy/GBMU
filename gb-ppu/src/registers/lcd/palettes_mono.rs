use super::super::Palette;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Default, Clone, Debug)]
pub struct PalettesMono {
    bg: Palette,
    obj_0: Palette,
    obj_1: Palette,
}

impl PalettesMono {
    pub const SIZE: usize = 3;
    pub const BACKGROUND: usize = 0;
    pub const OBJ_O: usize = 1;
    pub const OBJ_1: usize = 2;

    pub fn new() -> Self {
        PalettesMono {
            bg: Palette::new_background(),
            obj_0: Palette::new_sprite(),
            obj_1: Palette::new_sprite(),
        }
    }

    pub fn bg(&self) -> &Palette {
        &self.bg
    }

    pub fn obj(&self) -> (&Palette, &Palette) {
        (&self.obj_0, &self.obj_1)
    }

    pub fn read(&self, pos: usize) -> u8 {
        match pos {
            Self::BACKGROUND => self.bg.into(),
            Self::OBJ_O => self.obj_0.into(),
            Self::OBJ_1 => self.obj_1.into(),
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, pos: usize, val: u8) {
        match pos {
            Self::BACKGROUND => self.bg = Palette::new(val, false),
            Self::OBJ_O => self.obj_0 = Palette::new(val, true),
            Self::OBJ_1 => self.obj_1 = Palette::new(val, true),
            _ => {}
        }
    }
}

impl From<[u8; 3]> for PalettesMono {
    fn from(bytes: [u8; 3]) -> PalettesMono {
        PalettesMono {
            bg: Palette::new(bytes[Self::BACKGROUND], false),
            obj_0: Palette::new(bytes[Self::OBJ_O], true),
            obj_1: Palette::new(bytes[Self::OBJ_1], true),
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
