use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::{Tick, Ticker};
use sdl2::audio::AudioQueue;
use std::{cell::RefCell, rc::Rc};

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
    fn sweep_period(&self) {}
    fn negate(&self) {}
    fn shift(&self) {}
}

#[derive(PartialEq)]
enum Direction {
    Inc,
    Dec,
}
#[derive(Default)]
struct VolumeEnvelope {
    bits: u8,
}

impl VolumeEnvelope {
    fn initial_volume(&self) -> u8 {
        self.bits >> 4
    }
    fn envelope_direction(&self) -> Direction {
        if self.bits & 0x8 == 1 {
            Direction::Inc
        } else {
            Direction::Dec
        }
    }
    fn envelope_sweep_nb(&self) -> u8 {
        self.bits & 0x7
    }
}

struct SoundChannel {
    buffer: Vec<i16>,
    channel_type: ChannelType,
    sweep: Option<Sweep>,
    duty_pattern_index: Option<u8>,
    length_load: u8,
    volume_envelope: Option<VolumeEnvelope>,
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
            duty_pattern_index: if channel_type == ChannelType::SquareWave {
                Some(0)
            } else {
                None
            },
            length_load: 0,
            volume_envelope: if channel_type == ChannelType::SquareWave
                || channel_type == ChannelType::Noise
            {
                Some(VolumeEnvelope::default())
            } else {
                None
            },
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

impl<A> FileOperation<A, IORegArea> for SoundChannel
where
    A: Address<IORegArea>,
    u16: From<A>,
{
    fn read(&self, addr: A) -> Result<u8, Error> {
        use IORegArea::{Nr21, Nr22, Nr23, Nr24};
        match addr.area_type() {
            Nr21 => {
                let mut res = 0;
                if let Some(index) = self.duty_pattern_index {
                    res = index << 6;
                }
                res |= self.length_load;
                Ok(res)
            }
            Nr22 => {
                if let Some(volume_envelope) = &self.volume_envelope {
                    Ok(volume_envelope.bits)
                } else {
                    Ok(0)
                }
            }
            Nr23 => {
                if let Some(frequency) = self.frequency {
                    Ok(frequency as u8)
                } else {
                    Ok(0)
                }
            }
            Nr24 => {
                let mut res = 0;
                res |= if self.trigger { 0x80 } else { 0 };
                res |= if self.length_enabled { 0x40 } else { 0 };
                if let Some(frequency) = self.frequency {
                    res |= ((frequency >> 8) & 0x07) as u8;
                }

                Ok(res)
            }

            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        use IORegArea::{Nr21, Nr22, Nr23, Nr24};
        match addr.area_type() {
            Nr21 => {
                if self.channel_type == ChannelType::SquareWave {
                    self.duty_pattern_index = Some(v >> 6);
                    self.length_load = v & 0x3F;
                } else if self.channel_type == ChannelType::Noise {
                    self.length_load = v & 0x3F;
                } else {
                    self.length_load = v;
                }
            }
            Nr22 => {
                if let Some(ve) = &mut self.volume_envelope {
                    (*ve).bits = v;
                }
            }
            Nr23 => {
                if self.channel_type == ChannelType::SquareWave
                    || self.channel_type == ChannelType::WaveForm
                {
                    if let Some(frequency) = &mut self.frequency {
                        *frequency &= v as u16;
                    }
                }
            }
            Nr24 => {
                self.trigger = v & 0x80 != 0;
                self.length_enabled = v & 0x40 != 0;
                if self.channel_type == ChannelType::SquareWave
                    || self.channel_type == ChannelType::WaveForm
                {
                    if let Some(frequency) = &mut self.frequency {
                        *frequency &= ((v & 0x07) as u16) << 8;
                    }
                }
            }

            _ => return Err(Error::SegmentationFault(addr.into())),
        };
        Ok(())
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
