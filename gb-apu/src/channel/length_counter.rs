use crate::ChannelType;

pub struct LengthCounter {
    channel_type: ChannelType,
    pub enabled: bool,
    pub length_load: u8,
    pub counter: u16,
}

impl LengthCounter {
    pub fn new(channel_type: ChannelType) -> Self {
        Self {
            channel_type,
            length_load: 0,
            counter: 0,
            enabled: false,
        }
    }

    pub fn reached_zero(&self) -> bool {
        self.counter == 0
    }

    pub fn step(&mut self) -> bool {
        if self.counter > 0 && self.enabled {
            self.counter -= 1;

            if self.counter == 0 {
                return true;
            }
        }
        false
    }

    pub fn reload(&mut self) {
        if self.counter != 0 {
            return;
        }
        self.counter = if self.channel_type == ChannelType::WaveForm {
            0x100
        } else {
            0x40
        }
    }
}
