use crate::registers::Palette;
use crate::Color;

pub struct Pixel<'p> {
    color: u8,
    palette: &'p Palette,
    _sprite_priority: Option<u8>,
    _background_priority: bool,
}

impl<'p> From<Pixel<'p>> for Color {
    fn from(pixel: Pixel) -> Color {
        pixel.palette.get_color(pixel.color).unwrap_or_default()
    }
}
