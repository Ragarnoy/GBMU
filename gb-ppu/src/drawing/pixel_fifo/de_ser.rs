use super::super::pixel::de_ser::PixelDeSer;
use super::PixelFIFO;
use crate::registers::LcdReg;
use std::collections::VecDeque;
use std::ops::Deref;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct PixelFIFODeSer {
    pixels: VecDeque<PixelDeSer>,
    enabled: bool,
}

impl PixelFIFODeSer {
    pub fn from_fifo(fifo: PixelFIFO, lcd_reg: impl Deref<Target = LcdReg>) -> Self {
        let pixels = fifo
            .pixels
            .into_iter()
            .map(|p| PixelDeSer::from_pixel(p, &lcd_reg.pal_mono))
            .collect();
        PixelFIFODeSer {
            pixels,
            enabled: fifo.enabled,
        }
    }

    pub fn into_fifo(self, lcd_reg: impl Deref<Target = LcdReg>) -> PixelFIFO {
        let pixels = self
            .pixels
            .into_iter()
            .map(|p| p.into_pixel(&lcd_reg.pal_mono))
            .collect();
        PixelFIFO {
            pixels,
            enabled: self.enabled,
        }
    }
}
