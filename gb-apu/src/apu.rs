use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::{Tick, Ticker};
use sdl2::audio::AudioQueue;
use std::{cell::RefCell, rc::Rc};

use crate::{
    channel::sound_channel::SoundChannel, control::frame_sequencer::FrameSequencer, ChannelType,
    BUFFER_SIZE,
};

pub struct Apu {
    cycle_counter: u16,
    enabled: bool,
    audio_queue: Rc<RefCell<AudioQueue<f32>>>,
    buffer: [f32; BUFFER_SIZE],
    buffer_i: usize,
    sound_channels: Vec<SoundChannel>,
    frame_sequencer: FrameSequencer,
}

impl Apu {
    pub fn new(audio_queue: Rc<RefCell<AudioQueue<f32>>>) -> Apu {
        // Channels order in vector is important !
        let sound_channels = vec![
            SoundChannel::new(ChannelType::SquareWave, true),
            SoundChannel::new(ChannelType::SquareWave, false),
            SoundChannel::new(ChannelType::WaveForm, false),
            SoundChannel::new(ChannelType::Noise, false),
        ];
        Self {
            cycle_counter: 0,
            enabled: false,
            audio_queue,
            buffer: [0.0; BUFFER_SIZE],
            buffer_i: 0,
            sound_channels,
            frame_sequencer: FrameSequencer::default(),
        }
    }
    fn add_sample(&mut self) {
        let (left_sample, right_sample) = self.mix();
        self.buffer[self.buffer_i] = left_sample;
        self.buffer_i += 1;
        self.buffer[self.buffer_i] = right_sample;
        self.buffer_i += 1;
    }

    fn mix(&self) -> (f32, f32) {
        let mut sample = 0.0;

        for i in 0..self.sound_channels.len() {
            sample += self.sound_channels[i].get_dac_output();
        }
        sample /= self.sound_channels.len() as f32;
        (sample, sample)
    }

    fn queue_audio(&self) {
        self.audio_queue
            .borrow()
            .queue_audio(&self.buffer)
            .expect("failed to queue audio");
    }
}

impl Ticker for Apu {
    fn cycle_count(&self) -> Tick {
        Tick::TCycle
    }

    fn tick(&mut self, _addr_bus: &mut dyn Bus<u8>) {
        if !self.enabled {
            return;
        }
        self.cycle_counter += 1;
        for i in 0..self.sound_channels.len() {
            self.sound_channels[i].step();
        }

        if self.cycle_counter >= 0x2000 {
            self.cycle_counter %= 0x2000;

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

        if self.cycle_counter % 0x5F == 0 {
            self.add_sample();
        }
        if self.buffer_i >= BUFFER_SIZE {
            self.queue_audio();
            self.buffer_i = 0;
        }
    }
}

impl<A> FileOperation<A, IORegArea> for Apu
where
    A: Address<IORegArea>,
    u16: From<A>,
{
    fn read(&self, addr: A) -> Result<u8, Error> {
        use IORegArea::{
            Nr10, Nr11, Nr12, Nr13, Nr14, Nr21, Nr22, Nr23, Nr24, Nr30, Nr31, Nr32, Nr33, Nr34,
            Nr41, Nr42, Nr43, Nr44, Nr50, Nr51, Nr52,
        };
        match addr.area_type() {
            Nr10 | Nr11 | Nr12 | Nr13 | Nr14 => self.sound_channels[0].read(addr),
            Nr21 | Nr22 | Nr23 | Nr24 => self.sound_channels[1].read(addr),
            Nr30 | Nr31 | Nr32 | Nr33 | Nr34 => self.sound_channels[2].read(addr),
            Nr41 | Nr42 | Nr43 | Nr44 => self.sound_channels[3].read(addr),
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
        use IORegArea::{
            Nr10, Nr11, Nr12, Nr13, Nr14, Nr21, Nr22, Nr23, Nr24, Nr30, Nr31, Nr32, Nr33, Nr34,
            Nr41, Nr42, Nr43, Nr44, Nr50, Nr51, Nr52,
        };
        match addr.area_type() {
            Nr10 | Nr11 | Nr12 | Nr13 | Nr14 => return self.sound_channels[0].write(v, addr),
            Nr21 | Nr22 | Nr23 | Nr24 => return self.sound_channels[1].write(v, addr),
            Nr30 | Nr31 | Nr32 | Nr33 | Nr34 => return self.sound_channels[2].write(v, addr),
            Nr41 | Nr42 | Nr43 | Nr44 => return self.sound_channels[3].write(v, addr),
            Nr52 => {
                let was_enabled = self.enabled;
                let enabled = v & 0x80 != 0x00;
                if was_enabled && !enabled {
                    self.sound_channels = vec![
                        SoundChannel::new(ChannelType::SquareWave, true),
                        SoundChannel::new(ChannelType::SquareWave, false),
                        SoundChannel::new(ChannelType::WaveForm, false),
                        SoundChannel::new(ChannelType::Noise, false),
                    ];
                }
                self.enabled = enabled;
            }
            _ => return Err(Error::SegmentationFault(addr.into())),
        };
        Ok(())
    }
}
