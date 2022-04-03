use crate::{
    channel::duty::Duty,
    channel::length_counter::LengthCounter,
    channel::sweep::Sweep,
    channel::timer::Timer,
    channel::volume_envelope::{Direction, VolumeEnvelope},
    ChannelType,
};
use gb_bus::{Address, Error, FileOperation, IORegArea};

#[derive(Debug)]
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
                    duty.step();
                }
            }
        }
    }

    pub fn length_counter_step(&mut self) {
        self.enabled = self.length_counter.step();
    }

    pub fn sweep_step(&mut self) {
        if let Some(sweep) = &mut self.sweep {
            let reached_zero = sweep.step();
            if reached_zero && sweep.enabled && sweep.period > 0 {
                let new_frequency = sweep.calculate_frequency();
                if sweep.is_overflowing(new_frequency) {
                    self.enabled = false;
                } else if sweep.shift_nb > 0 {
                    if let Some(timer) = &mut self.timer {
                        (*timer).frequency = new_frequency;
                    }
                    sweep.shadow_frequency = new_frequency;
                    let new_frequency = sweep.calculate_frequency();
                    self.enabled = sweep.is_overflowing(new_frequency);
                };
            }
        }
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
        use IORegArea::{
            Nr10, Nr11, Nr12, Nr13, Nr14, Nr21, Nr22, Nr23, Nr24, Nr30, Nr31, Nr32, Nr33, Nr34,
            Nr41, Nr42, Nr43, Nr44, WaveRam0, WaveRam1, WaveRam2, WaveRam3, WaveRam4, WaveRam5,
            WaveRam6, WaveRam7, WaveRam8, WaveRam9, WaveRamA, WaveRamB, WaveRamC, WaveRamD,
            WaveRamE, WaveRamF,
        };
        match addr.area_type() {
            Nr10 => {
                if let Some(sweep) = &self.sweep {
                    let mut res = 0;
                    res |= (sweep.period & 0x7) << 4;
                    res |= match sweep.direction {
                        Direction::Dec => 0b1000,
                        Direction::Inc => 0,
                    };
                    res |= sweep.shift_nb & 0x7;
                    Ok(res)
                } else {
                    Ok(0)
                }
            }
            Nr11 | Nr21 => {
                let mut res = 0;
                if let Some(duty) = &self.duty {
                    res = duty.pattern_index << 6;
                }
                res |= self.length_counter.length_load;
                Ok(res)
            }
            Nr12 | Nr22 => {
                if let Some(ve) = &self.volume_envelope {
                    let mut res = 0;
                    res |= ve.initial_volume << 4;
                    res |= match ve.envelope_direction {
                        Direction::Inc => 0b1000,
                        Direction::Dec => 0,
                    };
                    res |= ve.period & 0x7;
                    Ok(res)
                } else {
                    Ok(0)
                }
            }
            Nr13 | Nr23 => {
                if let Some(timer) = &self.timer {
                    Ok(timer.frequency as u8)
                } else {
                    Ok(0)
                }
            }
            Nr14 | Nr24 => {
                let mut res = 0;
                res |= if self.enabled { 0x80 } else { 0 };
                res |= if self.length_counter.enabled { 0x40 } else { 0 };
                if let Some(timer) = &self.timer {
                    res |= ((timer.frequency >> 8) & 0x07) as u8;
                }

                Ok(res)
            }
            WaveRam0 | WaveRam1 | WaveRam2 | WaveRam3 | WaveRam4 | WaveRam5 | WaveRam6
            | WaveRam7 | WaveRam8 | WaveRam9 | WaveRamA | WaveRamB | WaveRamC | WaveRamD
            | WaveRamE | WaveRamF => Ok(0),

            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        use IORegArea::{
            Nr10, Nr11, Nr12, Nr13, Nr14, Nr21, Nr22, Nr23, Nr24, Nr30, Nr31, Nr32, Nr33, Nr34,
            Nr41, Nr42, Nr43, Nr44, WaveRam0, WaveRam1, WaveRam2, WaveRam3, WaveRam4, WaveRam5,
            WaveRam6, WaveRam7, WaveRam8, WaveRam9, WaveRamA, WaveRamB, WaveRamC, WaveRamD,
            WaveRamE, WaveRamF,
        };
        match addr.area_type() {
            Nr10 => {
                if let Some(sweep) = &mut self.sweep {
                    (*sweep).period = (v >> 4) & 0x7;
                    (*sweep).direction = if v & 0b1000 != 0 {
                        Direction::Dec
                    } else {
                        Direction::Inc
                    };
                    (*sweep).shift_nb = v & 0x7;
                }
            }
            Nr11 | Nr21 => {
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
            Nr12 | Nr22 => {
                if let Some(ve) = &mut self.volume_envelope {
                    (*ve).initial_volume = v >> 4;
                    (*ve).envelope_direction = if v & 0b1000 != 0 {
                        Direction::Inc
                    } else {
                        Direction::Dec
                    };
                    (*ve).period = v & 0x7;
                }
            }
            Nr13 | Nr23 => {
                if self.channel_type == ChannelType::SquareWave
                    || self.channel_type == ChannelType::WaveForm
                {
                    if let Some(timer) = &mut self.timer {
                        let high_byte = (*timer).frequency.to_le_bytes()[1];
                        (*timer).frequency = (high_byte as u16 & 0x7) << 8 | v as u16;
                    }
                }
            }
            Nr14 | Nr24 => {
                self.enabled = v & 0x80 != 0;
                self.length_counter.enabled = v & 0x40 != 0;
                if self.channel_type == ChannelType::SquareWave
                    || self.channel_type == ChannelType::WaveForm
                {
                    if let Some(timer) = &mut self.timer {
                        let low_byte = (*timer).frequency.to_le_bytes()[0];
                        (*timer).frequency = (v as u16 & 0x07) << 8 | low_byte as u16;
                    }
                }

                if self.enabled {
                    self.length_counter.reload();
                    if let Some(ve) = &mut self.volume_envelope {
                        (*ve).reload();
                    }
                    if let Some(sweep) = &mut self.sweep {
                        self.enabled = (*sweep).reload(self.timer.as_ref().unwrap().frequency);
                    }
                }
            }
            WaveRam0 | WaveRam1 | WaveRam2 | WaveRam3 | WaveRam4 | WaveRam5 | WaveRam6
            | WaveRam7 | WaveRam8 | WaveRam9 | WaveRamA | WaveRamB | WaveRamC | WaveRamD
            | WaveRamE | WaveRamF => {}

            _ => return Err(Error::SegmentationFault(addr.into())),
        };
        Ok(())
    }
}
