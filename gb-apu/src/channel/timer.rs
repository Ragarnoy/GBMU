use crate::ChannelType;
#[derive(Debug)]
pub struct Timer {
    channel_type: ChannelType,
    pub frequency: u16,
    counter: u32,
    pub divisor_code: u8,
    pub shift_amout: u8,
}

impl Timer {
    pub fn new(channel_type: ChannelType) -> Self {
        Self {
            channel_type,
            frequency: 0,
            counter: 0,
            divisor_code: 0,
            shift_amout: 0,
        }
    }

    pub fn step(&mut self) -> bool {
        if self.counter > 0 {
            self.counter -= 1;
            false
        } else {
            self.reload();
            true
        }
    }

    fn divisor(&self) -> u32 {
        match self.divisor_code {
            0 => 8,
            n => (n << 4) as u32,
        }
    }

    pub fn reload(&mut self) {
        self.counter = match self.channel_type {
            ChannelType::SquareWave => ((2048 - self.frequency) * 4) as u32,
            ChannelType::WaveForm => ((2048 - self.frequency) * 2) as u32,
            ChannelType::Noise => self.divisor() << self.shift_amout,
        }
    }
}
