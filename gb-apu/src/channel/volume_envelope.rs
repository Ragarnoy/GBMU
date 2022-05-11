#[derive(PartialEq, Debug)]
pub enum Direction {
    Inc,
    Dec,
}

#[derive(Debug)]
pub struct VolumeEnvelope {
    pub initial_volume: u8,
    pub envelope_direction: Direction,
    pub period: u8,
    pub volume: u8,
    counter: u8,
}

impl Default for VolumeEnvelope {
    fn default() -> Self {
        Self {
            initial_volume: 0,
            envelope_direction: Direction::Dec,
            period: 0,
            volume: 0,
            counter: 0,
        }
    }
}

impl VolumeEnvelope {
    pub fn step(&mut self) {
        if self.period == 0 {
            return;
        }
        if self.counter == 0 {
            self.counter = self.period;
            self.update_volume();
        } else {
            self.counter -= 1;
        }
    }

    fn update_volume(&mut self) {
        let next_volume = match self.envelope_direction {
            Direction::Inc => self.volume.wrapping_add(1),
            Direction::Dec => self.volume.wrapping_sub(1),
        };
        if next_volume <= 0xF {
            self.volume = next_volume;
        }
    }

    pub fn reload(&mut self) {
        self.volume = self.initial_volume;
        self.counter = self.period;
    }
}
