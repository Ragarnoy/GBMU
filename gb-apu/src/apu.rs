use gb_bus::{Address, Bus, Error, FileOperation, IORegArea};
use gb_clock::{Tick, Ticker};
use sdl2::audio::AudioQueue;
use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Clone, Copy)]
pub enum ChannelType {
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

const DUTY_CYCLES: [[bool; 8]; 4] = [
    [false, false, false, false, false, false, false, true],
    [true, false, false, false, false, false, false, true],
    [true, false, false, false, false, true, true, true],
    [false, true, true, true, true, true, true, false],
];

#[derive(Default)]
struct Duty {
    pattern_index: u8,
    step: u8,
}

impl Duty {
    pub fn reset(&mut self) {
        self.step = 0;
    }
    pub fn next(&mut self) -> bool {
        self.step += 1;
        self.step %= 8;
        DUTY_CYCLES[self.pattern_index as usize][self.step as usize]
    }
}

pub struct LengthCounter {
    channel_type: ChannelType,
    enabled: bool,
    length_load: u8,
    counter: u16,
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
        self.counter = if self.channel_type == ChannelType::WaveForm {
            0x100
        } else {
            0x40
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    Inc,
    Dec,
}
struct VolumeEnvelope {
    initial_volume: u8,
    envelope_direction: Direction,
    period: u8,
    volume: u8,
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
    fn step(&mut self) {
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

    fn reload(&mut self) {
        self.volume = self.initial_volume;
        self.counter = self.period;
    }
}

struct SoundChannel {
    buffer: Vec<i16>,
    channel_type: ChannelType,
    sweep: Option<Sweep>,
    duty: Option<Duty>,
    length_counter: LengthCounter,
    volume_envelope: Option<VolumeEnvelope>,
    enabled: bool,
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
            duty: if channel_type == ChannelType::SquareWave {
                Some(Duty::default())
            } else {
                None
            },
            length_counter: LengthCounter::new(channel_type.clone()),
            volume_envelope: if channel_type == ChannelType::SquareWave
                || channel_type == ChannelType::Noise
            {
                Some(VolumeEnvelope::default())
            } else {
                None
            },
            enabled: false,
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
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn length_counter_step(&mut self) {
        self.enabled = self.length_counter.step();
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
                if let Some(duty) = &self.duty {
                    res = duty.pattern_index << 6;
                }
                res |= self.length_counter.length_load;
                Ok(res)
            }
            Nr22 => {
                if let Some(ve) = &self.volume_envelope {
                    let mut res = 0;
                    res |= ve.initial_volume << 4;
                    res |= match ve.envelope_direction {
                        Direction::Inc => 1,
                        Direction::Dec => 0,
                    };
                    res |= ve.period & 0x7;
                    Ok(res)
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
                res |= if self.enabled { 0x80 } else { 0 };
                res |= if self.length_counter.enabled { 0x40 } else { 0 };
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
                    if let Some(duty) = &mut self.duty {
                        (*duty).pattern_index = v >> 6;
                        (*duty).step = 0;
                    }
                }
                if self.channel_type == ChannelType::SquareWave
                    || self.channel_type == ChannelType::Noise
                {
                    self.length_counter.length_load = v & 0x3F;
                    self.length_counter.counter = 0x40 - (v & 0x3F) as u16;
                } else {
                    self.length_counter.length_load = v;
                    self.length_counter.counter = 0x100 - v as u16;
                }
            }
            Nr22 => {
                if let Some(ve) = &mut self.volume_envelope {
                    (*ve).initial_volume = v >> 4;
                    (*ve).envelope_direction = if v & 0x8 == 1 {
                        Direction::Inc
                    } else {
                        Direction::Dec
                    };
                    (*ve).period = v & 0x7;
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
                self.enabled = v & 0x80 != 0;
                self.length_counter.enabled = v & 0x40 != 0;
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

#[derive(Default)]
struct FrameSequencer {
    step: u8,
}

impl FrameSequencer {
    fn next(&mut self) -> u8 {
        self.step += 1;
        self.step %= 8;
        self.step
    }
}

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
