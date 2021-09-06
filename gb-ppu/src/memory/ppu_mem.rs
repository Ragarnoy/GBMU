use super::{Oam, Vram};
use gb_bus::{Address, FileOperation};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPUMem {
    vram: Rc<RefCell<Vram>>,
    oam: Rc<RefCell<Oam>>,
}

impl PPUMem {
    pub fn new(vram: Rc<RefCell<Vram>>, oam: Rc<RefCell<Oam>>) -> Self {
        PPUMem { vram, oam }
    }
}
