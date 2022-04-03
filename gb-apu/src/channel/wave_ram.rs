const SAMPLES_NB: usize = 32;
#[derive(Default, Debug)]
pub struct ProgrammableWave {
    samples: [u8; SAMPLES_NB],
    step: usize,
    pub volume_shift: u8,
}

impl ProgrammableWave {
    pub fn step(&mut self) {
        if self.volume_shift != 0 {
            // dbg!(self.volume_shift);
        }
        self.step += 1;
        self.step %= SAMPLES_NB;
    }

    pub fn get_dac_input(&self) -> f32 {
        // dbg!(self.samples[self.step]);
        (self.samples[self.step] >> self.volume_shift) as f32
    }

    pub fn get_samples_at_index(&self, i: usize) -> u8 {
        self.samples[i] << 4 | self.samples[i + 1] & 0xF
    }

    pub fn set_samples_at_index(&mut self, i: usize, v: u8) {
        self.samples[i] = v >> 4;
        self.samples[i + 1] = v & 0xF;
    }

    pub fn reload(&mut self) {
        // TODO Wave channel's position is set to 0 but sample buffer is NOT refilled.
    }
}
