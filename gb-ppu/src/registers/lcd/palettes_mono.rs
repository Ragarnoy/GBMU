use super::super::Palette;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Default, Clone, Debug)]
pub struct PalettesMono {
    bg: Rc<Cell<Palette>>,
    obj_0: Rc<Cell<Palette>>,
    obj_1: Rc<Cell<Palette>>,
}

impl PalettesMono {
    pub const SIZE: usize = 3;
    pub const BACKGROUND: usize = 0;
    pub const OBJ_O: usize = 1;
    pub const OBJ_1: usize = 2;

    pub fn new() -> Self {
        PalettesMono {
            bg: Rc::new(Cell::new(Palette::new_background())),
            obj_0: Rc::new(Cell::new(Palette::new_sprite())),
            obj_1: Rc::new(Cell::new(Palette::new_sprite())),
        }
    }

    pub fn bg(&self) -> &Rc<Cell<Palette>> {
        &self.bg
    }

    pub fn obj(&self) -> (&Rc<Cell<Palette>>, &Rc<Cell<Palette>>) {
        (&self.obj_0, &self.obj_1)
    }

    pub fn read(&self, pos: usize) -> u8 {
        match pos {
            Self::BACKGROUND => self.bg.get().into(),
            Self::OBJ_O => self.obj_0.get().into(),
            Self::OBJ_1 => self.obj_1.get().into(),
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, pos: usize, val: u8) {
        match pos {
            Self::BACKGROUND => self.bg.set(Palette::new(val, false)),
            Self::OBJ_O => self.obj_0.set(Palette::new(val, true)),
            Self::OBJ_1 => self.obj_1.set(Palette::new(val, true)),
            _ => {}
        }
    }
}

impl From<[u8; 3]> for PalettesMono {
    fn from(bytes: [u8; 3]) -> PalettesMono {
        PalettesMono {
            bg: Rc::new(Cell::new(Palette::new(bytes[Self::BACKGROUND], false))),
            obj_0: Rc::new(Cell::new(Palette::new(bytes[Self::OBJ_O], true))),
            obj_1: Rc::new(Cell::new(Palette::new(bytes[Self::OBJ_1], true))),
        }
    }
}

impl From<PalettesMono> for [u8; 3] {
    fn from(register: PalettesMono) -> [u8; 3] {
        [
            register.bg.get().into(),
            register.obj_0.get().into(),
            register.obj_1.get().into(),
        ]
    }
}
