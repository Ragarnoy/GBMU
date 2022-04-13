use crate::registers::{LcdReg, PaletteRef};
use crate::Color;
use std::ops::Deref;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Copy)]
pub struct Pixel {
    pub color: u8,
    pub palette: Option<PaletteRef>,
    oam_index: Option<u8>,
    background_priority: bool,
}

impl Pixel {
    pub fn new(color: u8, palette: Option<PaletteRef>, background_priority: bool) -> Self {
        Pixel {
            color,
            palette,
            oam_index: None,
            background_priority,
        }
    }

    pub fn new_cgb(
        color: u8,
        palette: Option<PaletteRef>,
        background_priority: bool,
        oam_index: Option<u8>,
    ) -> Self {
        Pixel {
            color,
            palette,
            oam_index,
            background_priority,
        }
    }

    pub fn mix(&mut self, candidate: &Pixel) {
        if let Some(self_palette) = self.palette {
            if let Some(candidate_palette) = candidate.palette {
                if let (Some(self_index), Some(candidate_index)) =
                    (self.oam_index, candidate.oam_index)
                {
                    if candidate.color != 0 && candidate_index < self_index {
                        *self = *candidate;
                    }
                } else if !self_palette.is_sprite()
                    && candidate_palette.is_sprite()
                    && candidate.color != 0
                    && !(self.background_priority && self.color != 0)
                    && !(candidate.background_priority && self.color != 0)
                {
                    *self = *candidate;
                }
            }
        } else {
            *self = *candidate;
        }
    }

    pub fn overwrite(&mut self, candidate: &Pixel) {
        *self = *candidate;
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
