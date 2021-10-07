use super::Pixel;
use std::collections::VecDeque;

pub struct PixelFIFO {
    fifo: VecDeque<Pixel>,
}

impl PixelFIFO {
    pub fn new() -> Self {
        PixelFIFO {
            fifo: VecDeque::with_capacity(16),
        }
    }

    pub fn pop(&mut self) -> Option<Pixel> {
        if self.fifo.len() > 8 {
            self.fifo.pop_front()
        } else {
            None
        }
    }
}
