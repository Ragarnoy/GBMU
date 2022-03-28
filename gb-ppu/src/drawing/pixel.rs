use crate::registers::{LcdReg, PaletteRef};
use crate::Color;
use std::ops::Deref;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug)]
pub struct Pixel {
    pub color: u8,
    pub palette: Option<PaletteRef>,
    _sprite_priority: Option<u8>,
    background_priority: bool,
}

impl Pixel {
    pub fn new(color: u8, palette: Option<PaletteRef>, background_priority: bool) -> Self {
        Pixel {
            color,
            palette,
            _sprite_priority: None,
            background_priority,
        }
    }

    pub fn mix(&mut self, other: &Pixel) {
        if let Some(self_palette) = &self.palette {
            if let Some(other_palette) = &other.palette {
                if !self_palette.is_sprite()
                    && other_palette.is_sprite()
                    && other.color != 0
                    && !(other.background_priority && self.color != 0)
                {
                    *self = other.clone();
                }
            }
        } else {
            *self = other.clone();
        }
    }

    pub fn into_color(self, lcd_reg: &dyn Deref<Target = LcdReg>) -> Color {
        if let Some(self_palette) = self.palette {
            self_palette
                .get_color(lcd_reg, self.color)
                .unwrap_or_default()
        } else {
            Color::default()
        }
    }
}
