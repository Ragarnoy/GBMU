use super::super::LcdReg;
use crate::error::PPUResult;
use crate::Color;
use std::ops::Deref;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PaletteRef {
    MonoBgWin,
    MonoSprite0,
    MonoSprite1,
    CgbBGWin(u8),
    CgbSprite(u8),
}

impl PaletteRef {
    pub fn is_sprite(&self) -> bool {
        PaletteRef::MonoBgWin != *self
    }

    pub fn get_color(
        self,
        lcd_reg: &dyn Deref<Target = LcdReg>,
        color_value: u8,
    ) -> PPUResult<Color> {
        match self {
            PaletteRef::MonoBgWin => lcd_reg.deref().pal_mono.bg().get_color(color_value),
            PaletteRef::MonoSprite0 => lcd_reg.deref().pal_mono.obj().0.get_color(color_value),
            PaletteRef::MonoSprite1 => lcd_reg.deref().pal_mono.obj().1.get_color(color_value),
            PaletteRef::CgbBGWin(index) => {
                lcd_reg
                    .deref()
                    .pal_cgb
                    .get_color(color_value as usize, index as usize, true)
            }
            PaletteRef::CgbSprite(index) => {
                lcd_reg
                    .deref()
                    .pal_cgb
                    .get_color(color_value as usize, index as usize, false)
            }
        }
    }
}
