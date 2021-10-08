use super::Pixel;
use std::collections::VecDeque;

#[derive(Default)]
pub struct PixelFIFO {
    pixels: VecDeque<Pixel>,
    pub enabled: bool,
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

    #[allow(dead_code)]
    pub fn append(&mut self, mut new_pixels: VecDeque<Pixel>) -> Option<VecDeque<Pixel>> {
        if self.pixels.len() <= 8 && new_pixels.len() == 8 {
            self.pixels.append(&mut new_pixels);
            None
        } else {
            Some(new_pixels)
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
    fn append() {
        let palette = Rc::new(Cell::new(Palette::new()));
        let pixels = VecDeque::from_iter([
            Pixel::new(0, palette.clone(), false, false),
            Pixel::new(1, palette.clone(), false, false),
            Pixel::new(2, palette.clone(), false, false),
            Pixel::new(3, palette.clone(), false, false),
            Pixel::new(0, palette.clone(), false, false),
            Pixel::new(1, palette.clone(), false, false),
            Pixel::new(2, palette.clone(), false, false),
            Pixel::new(3, palette.clone(), false, false),
        ]);
        let mut fifo = PixelFIFO::new();

        fifo.append(pixels);
        assert_eq!(fifo.pixels.len(), 8, "incorrect pixel amount pushed");
        for (i, pixel) in fifo.pixels.iter().enumerate() {
            assert_eq!(pixel.color as usize, i % 4, "pixel order");
        }
    }

    #[test]
    fn pop() {
        let palette = Rc::new(Cell::new(Palette::new()));
        let pixels_0 = VecDeque::from_iter([
            Pixel::new(0, palette.clone(), false, false),
            Pixel::new(1, palette.clone(), false, false),
            Pixel::new(0, palette.clone(), false, false),
            Pixel::new(1, palette.clone(), false, false),
            Pixel::new(0, palette.clone(), false, false),
            Pixel::new(1, palette.clone(), false, false),
            Pixel::new(0, palette.clone(), false, false),
            Pixel::new(1, palette.clone(), false, false),
        ]);
        let pixels_1 = VecDeque::from_iter([
            Pixel::new(2, palette.clone(), false, false),
            Pixel::new(3, palette.clone(), false, false),
            Pixel::new(2, palette.clone(), false, false),
            Pixel::new(3, palette.clone(), false, false),
            Pixel::new(2, palette.clone(), false, false),
            Pixel::new(3, palette.clone(), false, false),
            Pixel::new(2, palette.clone(), false, false),
            Pixel::new(3, palette.clone(), false, false),
        ]);
        let mut fifo = PixelFIFO::new();

        fifo.append(pixels_0);
        assert!(fifo.pop().is_none(), "pop should have been blocked");
        fifo.append(pixels_1);
        assert!(fifo.pop().is_some(), "pop should not have been blocked");
        assert!(fifo.pop().is_some(), "pop should not have been blocked");
        assert_eq!(fifo.pixels.len(), 14, "incorrect pixel amount pushed");
        for (i, pixel) in fifo.pixels.iter().enumerate() {
            if i < 6 {
                assert_eq!(pixel.color as usize, i % 2, "pixel order");
            }
            if i >= 6 {
                assert_eq!(pixel.color as usize, i % 2 + 2, "pixel order");
            }
        }
    }
}
