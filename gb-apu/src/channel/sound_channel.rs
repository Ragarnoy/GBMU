use crate::{
    channel::duty::Duty,
    channel::length_counter::LengthCounter,
    channel::sweep::Sweep,
    channel::timer::Timer,
    channel::volume_envelope::{Direction, VolumeEnvelope},
    ChannelType,
};
use gb_bus::{Address, Error, FileOperation, IORegArea};

pub struct SoundChannel {
    pub channel_type: ChannelType,
    pub sweep: Option<Sweep>,
    pub duty: Option<Duty>,
    pub length_counter: LengthCounter,
    pub volume_envelope: Option<VolumeEnvelope>,
    pub enabled: bool,
    pub timer: Option<Timer>,
}

impl SoundChannel {
    pub fn new(channel_type: ChannelType, handles_sweep: bool) -> Self {
        SoundChannel {
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
            timer: if channel_type == ChannelType::SquareWave
                || channel_type == ChannelType::WaveForm
            {
                Some(Timer::new(channel_type))
            } else {
                None
            },
            channel_type,
        }
    }

    pub fn step(&mut self) {
        if let Some(timer) = &mut self.timer {
            if let Some(duty) = &mut self.duty {
                let reached_zero = timer.step();
                if reached_zero {
                    duty.step()
                }
            }
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn length_counter_step(&mut self) {
        self.enabled = self.length_counter.step();
    }

    pub fn get_dac_output(&self) -> f32 {
        if let Some(volume_envelope) = &self.volume_envelope {
            if volume_envelope.initial_volume == 0
                && volume_envelope.envelope_direction == Direction::Dec
            {
                return 0.0;
            }
            if let Some(duty) = &self.duty {
                let dac_input = duty.get_amplitude() * volume_envelope.volume;
                let dac_output = (dac_input as f32 / 7.5) - 1.0;
                return dac_output;
            }
        }
        0.0
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
                if let Some(timer) = &self.timer {
                    Ok(timer.frequency as u8)
                } else {
                    Ok(0)
                }
            }
            Nr24 => {
                let mut res = 0;
                res |= if self.enabled { 0x80 } else { 0 };
                res |= if self.length_counter.enabled { 0x40 } else { 0 };
                if let Some(timer) = &self.timer {
                    res |= ((timer.frequency >> 8) & 0x07) as u8;
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
                    if let Some(timer) = &mut self.timer {
                        (*timer).frequency &= v as u16;
                    }
                }
            }
            Nr24 => {
                self.enabled = v & 0x80 != 0;
                self.length_counter.enabled = v & 0x40 != 0;
                if self.channel_type == ChannelType::SquareWave
                    || self.channel_type == ChannelType::WaveForm
                {
                    if let Some(timer) = &mut self.timer {
                        (*timer).frequency &= ((v & 0x07) as u16) << 8;
                    }
                }
            }

            _ => return Err(Error::SegmentationFault(addr.into())),
        };
        Ok(())
    }
}
