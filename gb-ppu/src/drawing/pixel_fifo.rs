use super::Pixel;
use std::collections::VecDeque;

#[derive(Default)]
pub struct PixelFIFO {
    fifo: VecDeque<Pixel>,
    pub enabled: bool,
}

impl PixelFIFO {
    pub fn new() -> Self {
        PixelFIFO {
            fifo: VecDeque::with_capacity(16),
            enabled: true,
        }
    }

    pub fn pop(&mut self) -> Option<Pixel> {
        if self.fifo.len() > 8 {
            self.fifo.pop_front()
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.fifo.clear();
    }
}
