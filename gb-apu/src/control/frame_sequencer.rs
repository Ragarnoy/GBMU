#[derive(Default)]
pub struct FrameSequencer {
    step: u8,
}

impl FrameSequencer {
    pub fn next(&mut self) -> u8 {
        self.step = (self.step + 1) % 8;
        self.step
    }
}
