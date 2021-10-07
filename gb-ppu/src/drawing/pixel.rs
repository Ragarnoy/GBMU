use crate::registers::Palette;
use crate::Color;
use std::cell::Cell;
use std::rc::Rc;

pub struct Pixel {
    pub color: u8,
    pub palette: Rc<Cell<Palette>>,
    _sprite_priority: Option<u8>,
    _background_priority: bool,
}

impl Pixel {
    pub fn new(color: u8, palette: Rc<Cell<Palette>>, _background_priority: bool) -> Self {
        Pixel {
            color,
            palette,
            _sprite_priority: None,
            _background_priority,
        }
    }
}

impl From<Pixel> for Color {
    fn from(pixel: Pixel) -> Color {
        pixel
            .palette
            .get()
            .get_color(pixel.color)
            .unwrap_or_default()
    }
}
