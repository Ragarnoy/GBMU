use crate::ChannelType;
#[derive(Debug)]
pub struct Timer {
    channel_type: ChannelType,
    pub frequency: u16,
    counter: u16,
}

impl Timer {
    pub fn new(channel_type: ChannelType) -> Self {
        Self {
            channel_type,
            frequency: 0,
            counter: 0,
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

    pub fn reload(&mut self) {
        self.counter = match self.channel_type {
            ChannelType::SquareWave => (2048 - self.frequency) * 4,
            ChannelType::WaveForm => (2048 - self.frequency) * 2,
            _ => 0,
        }
    }
}
