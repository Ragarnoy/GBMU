use crate::registers::Palette;
use crate::Color;
use std::cell::Cell;
use std::rc::Rc;

pub struct Pixel {
    pub color: u8,
    pub palette: Rc<Cell<Palette>>,
    is_sprite: bool,
    _sprite_priority: Option<u8>,
    background_priority: bool,
}

impl Pixel {
    pub fn new(
        color: u8,
        palette: Rc<Cell<Palette>>,
        is_sprite: bool,
        background_priority: bool,
    ) -> Self {
        Pixel {
            color,
            palette,
            is_sprite,
            _sprite_priority: None,
            background_priority,
        }
    }

    pub fn mix(&mut self, other: Pixel) {
        if !self.is_sprite && (!other.background_priority && other.is_sprite && other.color != 0)
            || (other.background_priority && other.is_sprite && self.color != 0)
        {
            *self = other;
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
