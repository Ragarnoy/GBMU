use std::{cell::RefCell, rc::Rc};

use gb_bus::Bus;
use gb_clock::{Tick, Ticker};
use sdl2::audio::AudioQueue;

enum ChannelType {
    SquareWave,
    WaveForm,
    Noise,
}

#[derive(Default)]
struct Sweep {
    bits: u8,
}

impl Sweep {
    fn sweep_period() {}
    fn negate() {}
    fn shift() {}
}

struct SoundChannel {
    buffer: Vec<i16>,
    channel_type: ChannelType,
    sweep: Option<Sweep>,
}

impl SoundChannel {
    pub fn new(channel_type: ChannelType, sweep: Option<Sweep>) -> Self {
        SoundChannel {
            buffer: Vec::new(),
            channel_type,
            sweep,
        }
    }
}

pub struct Apu {
    audio_queue: Rc<RefCell<AudioQueue<i16>>>,
    mix_buffer: Vec<i16>,
    sound_channels: Vec<SoundChannel>,
}

impl Apu {
    pub fn new(audio_queue: Rc<RefCell<AudioQueue<i16>>>) -> Apu {
        let sound_channels = vec![
            SoundChannel::new(ChannelType::SquareWave, Some(Sweep::default())),
            SoundChannel::new(ChannelType::SquareWave, None),
            SoundChannel::new(ChannelType::WaveForm, None),
            SoundChannel::new(ChannelType::Noise, None),
        ];
        Self {
            audio_queue,
            mix_buffer: Vec::new(),
            sound_channels,
        }
    }
}

impl Ticker for Apu {
    fn cycle_count(&self) -> Tick {
        Tick::TCycle
    }

    fn tick(&mut self, _addr_bus: &mut dyn Bus<u8>) {
        self.audio_queue
            .borrow()
            .queue_audio(&self.mix_buffer)
            .expect("failed to queue audio");
    }
}
