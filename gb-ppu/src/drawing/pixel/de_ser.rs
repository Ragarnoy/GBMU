use super::Pixel;
use crate::ppu::de_ser::MonoPaletteRef;
use crate::registers::PalettesMono;
use std::ops::Deref;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PixelDeSer {
    color: u8,
    palette: MonoPaletteRef,
    sprite_priority: Option<u8>,
    background_priority: bool,
}

impl PixelDeSer {
    pub fn from_pixel(pixel: Pixel, pal_mono: impl Deref<Target = PalettesMono>) -> Self {
        let palette = if pixel.palette.get().is_sprite() {
            MonoPaletteRef::BgWin
        } else if pixel.palette.get().get_map() != pal_mono.obj().0.get().get_map() {
            MonoPaletteRef::Sprite1
        } else {
            MonoPaletteRef::Sprite0
        };
        PixelDeSer {
            color: pixel.color,
            palette,
            sprite_priority: pixel._sprite_priority,
            background_priority: pixel.background_priority,
        }
    }

    pub fn into_pixel(self, pal_mono: impl Deref<Target = PalettesMono>) -> Pixel {
        let palette = match self.palette {
            MonoPaletteRef::BgWin => pal_mono.bg().clone(),
            MonoPaletteRef::Sprite0 => pal_mono.obj().0.clone(),
            MonoPaletteRef::Sprite1 => pal_mono.obj().1.clone(),
        };
        Pixel {
            color: self.color,
            palette,
            _sprite_priority: self.sprite_priority,
            background_priority: self.background_priority,
        }
    }
}
