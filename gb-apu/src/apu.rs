use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::{Tick, Ticker};
use sdl2::audio::AudioQueue;
use std::{cell::RefCell, rc::Rc};

use crate::{
    channels::sound_channel::SoundChannel, control::frame_sequencer::FrameSequencer, ChannelType,
};

pub struct Apu {
    cycle_counter: u16,
    enabled: bool,
    audio_queue: Rc<RefCell<AudioQueue<i16>>>,
    mix_buffer: Vec<i16>,
    sound_channels: Vec<SoundChannel>,
    frame_sequencer: FrameSequencer,
}

impl Apu {
    pub fn new(audio_queue: Rc<RefCell<AudioQueue<i16>>>) -> Apu {
        // Channels order in vector is important !
        let sound_channels = vec![
            // SoundChannel::new(ChannelType::SquareWave, true),
            SoundChannel::new(ChannelType::SquareWave, false),
            // SoundChannel::new(ChannelType::WaveForm, false),
            // SoundChannel::new(ChannelType::Noise, false),
        ];
        Self {
            cycle_counter: 0,
            enabled: false,
            audio_queue,
            mix_buffer: Vec::new(),
            sound_channels,
            frame_sequencer: FrameSequencer::default(),
        }
    }

    fn queue_audio(&self) {
        self.audio_queue
            .borrow()
            .queue_audio(&self.mix_buffer)
            .expect("failed to queue audio");
    }
}

impl Ticker for Apu {
    fn cycle_count(&self) -> Tick {
        Tick::TCycle
    }

    fn tick(&mut self, _addr_bus: &mut dyn Bus<u8>) {
        if !self.enabled || self.cycle_counter < 0x2000 {
            self.cycle_counter += 1;
            return;
        }
        self.cycle_counter = 0;

        let step = self.frame_sequencer.next();
        if step == 0 || step == 2 || step == 4 || step == 6 {
            for i in 0..self.sound_channels.len() {
                self.sound_channels[i].length_counter_step();
            }
        }
        if step == 2 || step == 6 {
            for i in 0..self.sound_channels.len() {
                if let Some(ve) = &mut self.sound_channels[i].volume_envelope {
                    (*ve).step();
                }
            }
        }
    }
}

impl<A> FileOperation<A, IORegArea> for Apu
where
    A: Address<IORegArea>,
    u16: From<A>,
{
    fn read(&self, addr: A) -> Result<u8, Error> {
        use IORegArea::{Nr50, Nr51, Nr52};
        match addr.area_type() {
            Nr52 => Ok(if self.enabled { 0x80 } else { 0 }
                | if self.sound_channels[3].enabled {
                    0x8
                } else {
                    0
                }
                | if self.sound_channels[2].enabled {
                    0x4
                } else {
                    0
                }
                | if self.sound_channels[1].enabled {
                    0x2
                } else {
                    0
                }
                | if self.sound_channels[0].enabled {
                    0x1
                } else {
                    0
                }),
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        use IORegArea::{Nr50, Nr51, Nr52};
        match addr.area_type() {
            Nr52 => self.enabled = v & 0x80 != 0x00,
            _ => return Err(Error::SegmentationFault(addr.into())),
        };
        Ok(())
    }
}
