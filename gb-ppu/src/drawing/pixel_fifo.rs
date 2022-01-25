pub mod de_ser;

use super::Pixel;
use std::collections::VecDeque;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct PixelFIFO {
    pixels: VecDeque<Pixel>,
    pub enabled: bool,
}

impl Default for PixelFIFO {
    fn default() -> PixelFIFO {
        PixelFIFO::new()
    }
}

impl PixelFIFO {
    pub fn new() -> Self {
        PixelFIFO {
            pixels: VecDeque::with_capacity(16),
            enabled: true,
        }
    }

    pub fn pop(&mut self) -> Option<Pixel> {
        if self.pixels.len() > 8 {
            self.pixels.pop_front()
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.pixels.clear();
    }

    pub fn count(&self) -> usize {
        self.pixels.len()
    }

    pub fn append(&mut self, new_pixels: &mut VecDeque<Pixel>) -> bool {
        if self.pixels.len() <= 8 && new_pixels.len() == 8 {
            self.pixels.append(new_pixels);
            true
        } else {
            false
        }
    }

    pub fn mix(&mut self, mix_pixels: &VecDeque<Pixel>) -> bool {
        if self.pixels.len() >= 8 && mix_pixels.len() == 8 {
            for (in_place, mix_pixel) in self.pixels.iter_mut().zip(mix_pixels) {
                in_place.mix(mix_pixel);
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Pixel, PixelFIFO};
    use crate::registers::Palette;
    use std::cell::Cell;
    use std::collections::VecDeque;
    use std::iter::FromIterator;
    use std::rc::Rc;

    #[test]
    fn mix() {
        let pal_0 = Rc::new(Cell::new(Palette::new_background()));
        let mut pixels_0 = VecDeque::from_iter([
            Pixel::new(1, pal_0.clone(), false),
            Pixel::new(1, pal_0.clone(), false),
            Pixel::new(1, pal_0.clone(), false),
            Pixel::new(1, pal_0.clone(), false),
            Pixel::new(1, pal_0.clone(), false),
            Pixel::new(1, pal_0.clone(), false),
            Pixel::new(1, pal_0.clone(), false),
            Pixel::new(1, pal_0, false),
        ]);
        let pal_1 = Rc::new(Cell::new(Palette::new_sprite()));
        let pixels_1 = VecDeque::from_iter([
            Pixel::new(2, pal_1.clone(), false),
            Pixel::new(0, pal_1.clone(), false),
            Pixel::new(2, pal_1.clone(), false),
            Pixel::new(0, pal_1.clone(), false),
            Pixel::new(2, pal_1.clone(), false),
            Pixel::new(0, pal_1.clone(), false),
            Pixel::new(2, pal_1.clone(), false),
            Pixel::new(0, pal_1, false),
        ]);
        let mut fifo = PixelFIFO::new();

        fifo.append(&mut pixels_0);
        fifo.mix(&pixels_1);
        assert_eq!(fifo.pixels.len(), 8, "incorrect pixel amount pushed");
        for (i, pixel) in fifo.pixels.iter().enumerate() {
            if i % 2 == 0 {
                assert!(pixel.palette.get().is_sprite(), "pixel mixing failed");
            } else {
                assert!(!pixel.palette.get().is_sprite(), "pixel mixing failed");
            }
        }
    }

    #[test]
    fn append() {
        let palette = Rc::new(Cell::new(Palette::new_background()));
        let mut pixels = VecDeque::from_iter([
            Pixel::new(0, palette.clone(), false),
            Pixel::new(1, palette.clone(), false),
            Pixel::new(2, palette.clone(), false),
            Pixel::new(3, palette.clone(), false),
            Pixel::new(0, palette.clone(), false),
            Pixel::new(1, palette.clone(), false),
            Pixel::new(2, palette.clone(), false),
            Pixel::new(3, palette, false),
        ]);
        let mut fifo = PixelFIFO::new();

        fifo.append(&mut pixels);
        assert_eq!(fifo.pixels.len(), 8, "incorrect pixel amount pushed");
        for (i, pixel) in fifo.pixels.iter().enumerate() {
            assert_eq!(pixel.color as usize, i % 4, "pixel order");
        }
    }

    #[test]
    fn pop() {
        let palette = Rc::new(Cell::new(Palette::new_background()));
        let mut pixels_0 = VecDeque::from_iter([
            Pixel::new(0, palette.clone(), false),
            Pixel::new(1, palette.clone(), false),
            Pixel::new(0, palette.clone(), false),
            Pixel::new(1, palette.clone(), false),
            Pixel::new(0, palette.clone(), false),
            Pixel::new(1, palette.clone(), false),
            Pixel::new(0, palette.clone(), false),
            Pixel::new(1, palette.clone(), false),
        ]);
        let mut pixels_1 = VecDeque::from_iter([
            Pixel::new(2, palette.clone(), false),
            Pixel::new(3, palette.clone(), false),
            Pixel::new(2, palette.clone(), false),
            Pixel::new(3, palette.clone(), false),
            Pixel::new(2, palette.clone(), false),
            Pixel::new(3, palette.clone(), false),
            Pixel::new(2, palette.clone(), false),
            Pixel::new(3, palette, false),
        ]);
        let mut fifo = PixelFIFO::new();

        fifo.append(&mut pixels_0);
        assert!(fifo.pop().is_none(), "pop should have been blocked");
        fifo.append(&mut pixels_1);
        assert!(fifo.pop().is_some(), "pop should not have been blocked");
        assert_eq!(fifo.pixels.len(), 15, "incorrect pixel amount pushed");
        for (i, pixel) in fifo.pixels.iter().enumerate() {
            if i < 7 {
                assert_eq!(pixel.color as usize, (i + 1) % 2, "pixel order");
            } else {
                assert_eq!(pixel.color as usize, (i + 1) % 2 + 2, "pixel order");
            }
        }
    }
}
