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

    pub fn new() -> Self {
        PalettesMono {
            bg: Rc::new(Cell::new(Palette::new(false))),
            obj_0: Rc::new(Cell::new(Palette::new(true))),
            obj_1: Rc::new(Cell::new(Palette::new(true))),
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
            0 => self.bg.get().into(),
            1 => self.obj_0.get().into(),
            2 => self.obj_1.get().into(),
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, pos: usize, val: u8) {
        match pos {
            0 => self.bg.set(val.into()),
            1 => self.obj_0.set(val.into()),
            2 => self.obj_1.set(val.into()),
            _ => {}
        }
    }
}

impl From<[u8; 3]> for PalettesMono {
    fn from(bytes: [u8; 3]) -> PalettesMono {
        PalettesMono {
            bg: Rc::new(Cell::new(bytes[0].into())),
            obj_0: Rc::new(Cell::new(bytes[1].into())),
            obj_1: Rc::new(Cell::new(bytes[2].into())),
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
