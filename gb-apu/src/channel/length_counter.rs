use crate::ChannelType;

#[derive(Debug)]
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

    pub fn step(&mut self) {
        if self.counter > 0 && self.enabled {
            self.counter -= 1;
        }
    }

    pub fn should_disabled_channel(&self) -> bool {
        self.counter == 0
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

        //TODO impl next step will not count (obscure behavior)
    }
}
