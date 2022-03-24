use super::Pixel;
use std::collections::VecDeque;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone)]
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
    use crate::registers::PaletteRef;
    use std::collections::VecDeque;
    use std::iter::FromIterator;

    #[test]
    fn mix() {
        let mut pixels_0 = VecDeque::from_iter([
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
        ]);
        let pixels_1 = VecDeque::from_iter([
            Pixel::new(2, Some(PaletteRef::MonoSprite0), false),
            Pixel::new(0, Some(PaletteRef::MonoSprite0), false),
            Pixel::new(2, Some(PaletteRef::MonoSprite0), false),
            Pixel::new(0, Some(PaletteRef::MonoSprite0), false),
            Pixel::new(2, Some(PaletteRef::MonoSprite0), false),
            Pixel::new(0, Some(PaletteRef::MonoSprite0), false),
            Pixel::new(2, Some(PaletteRef::MonoSprite0), false),
            Pixel::new(0, Some(PaletteRef::MonoSprite0), false),
        ]);
        let mut fifo = PixelFIFO::new();

        fifo.append(&mut pixels_0);
        fifo.mix(&pixels_1);
        assert_eq!(fifo.pixels.len(), 8, "incorrect pixel amount pushed");
        for (i, pixel) in fifo.pixels.iter().enumerate() {
            if i % 2 == 0 {
                assert!(
                    pixel.palette.as_ref().unwrap().is_sprite(),
                    "pixel mixing failed"
                );
            } else {
                assert!(
                    !pixel.palette.as_ref().unwrap().is_sprite(),
                    "pixel mixing failed"
                );
            }
        }
    }

    #[test]
    fn append() {
        let mut pixels = VecDeque::from_iter([
            Pixel::new(0, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(2, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(3, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(0, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(2, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(3, Some(PaletteRef::MonoBgWin), false),
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
        let mut pixels_0 = VecDeque::from_iter([
            Pixel::new(0, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(0, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(0, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(0, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(1, Some(PaletteRef::MonoBgWin), false),
        ]);
        let mut pixels_1 = VecDeque::from_iter([
            Pixel::new(2, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(3, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(2, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(3, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(2, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(3, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(2, Some(PaletteRef::MonoBgWin), false),
            Pixel::new(3, Some(PaletteRef::MonoBgWin), false),
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
