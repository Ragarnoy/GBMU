use std::{cell::RefCell, rc::Rc};

use gb_bus::Bus;
use gb_clock::{Tick, Ticker};
use sdl2::audio::AudioQueue;

#[derive(PartialEq)]
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

enum Direction {
    Inc,
    Dec,
}
struct VolumeEnvelope {
    bits: u8,
}

impl VolumeEnvelope {
    fn initial_volume() -> u8 {
        0
    }
    fn envelope_direction() -> Direction {
        Direction::Inc
    }
    fn envelope_sweep_nb() -> u8 {
        0
    }
}

struct SoundChannel {
    buffer: Vec<i16>,
    channel_type: ChannelType,
    sweep: Option<Sweep>,
    duty_pattern: Option<u8>,
    length_load: u16,
    trigger: bool,
    length_enabled: bool,
    frequency: Option<u16>,
}

impl SoundChannel {
    pub fn new(channel_type: ChannelType, handles_sweep: bool) -> Self {
        SoundChannel {
            buffer: Vec::new(),
            sweep: if handles_sweep && channel_type == ChannelType::SquareWave {
                Some(Sweep::default())
            } else {
                None
            },
            duty_pattern: if channel_type == ChannelType::SquareWave {
                Some(0)
            } else {
                None
            },
            length_load: 0,
            trigger: false,
            length_enabled: false,
            frequency: if channel_type == ChannelType::SquareWave
                || channel_type == ChannelType::WaveForm
            {
                Some(0)
            } else {
                None
            },
            channel_type,
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
            // SoundChannel::new(ChannelType::SquareWave, Some(Sweep::default())),
            SoundChannel::new(ChannelType::SquareWave, false),
            // SoundChannel::new(ChannelType::WaveForm, None),
            // SoundChannel::new(ChannelType::Noise, None),
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
