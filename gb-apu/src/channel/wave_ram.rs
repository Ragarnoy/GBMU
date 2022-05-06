const SAMPLES_NB: usize = 32;
#[derive(Default, Debug, Clone)]
pub struct ProgrammableWave {
    samples: [u8; SAMPLES_NB],
    step: usize,
    pub bits: u8,
}

impl ProgrammableWave {
    pub fn step(&mut self) {
        self.step = (self.step + 1) % SAMPLES_NB;
    }

    fn volume_shift(&self) -> u8 {
        match (self.bits & 0b0110_0000) >> 5 {
            0b00 => 4, // mute
            0b01 => 0, // 100%
            0b10 => 1, // 50%
            0b11 => 2, // 25%
            _ => unreachable!(),
        }
    }

    pub fn get_dac_input(&self) -> f32 {
        (self.samples[self.step] >> self.volume_shift()) as f32
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
