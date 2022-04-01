#[derive(Default)]
pub struct FrameSequencer {
    step: u8,
}

impl FrameSequencer {
    pub fn next(&mut self) -> u8 {
        self.step += 1;
        self.step %= 8;
        self.step
    }
}
