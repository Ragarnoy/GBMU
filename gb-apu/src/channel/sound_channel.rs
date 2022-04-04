use crate::{
    channel::duty::Duty,
    channel::length_counter::LengthCounter,
    channel::sweep::Sweep,
    channel::timer::Timer,
    channel::volume_envelope::{Direction, VolumeEnvelope},
    ChannelType,
};
use gb_bus::{io_reg_constant::WAVE_RAM_0, Address, Error, FileOperation, IORegArea};

use super::wave_ram::ProgrammableWave;

#[derive(Debug)]
pub struct SoundChannel {
    pub enabled: bool,
    pub channel_type: ChannelType,
    sweep: Option<Sweep>,
    duty: Option<Duty>,
    length_counter: LengthCounter,
    volume_envelope: Option<VolumeEnvelope>,
    timer: Option<Timer>,
    programmable_wave: Option<ProgrammableWave>,
}

impl SoundChannel {
    pub fn new(channel_type: ChannelType, handles_sweep: bool) -> Self {
        SoundChannel {
            enabled: false,
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
            length_counter: LengthCounter::new(channel_type),
            volume_envelope: if channel_type == ChannelType::SquareWave
                || channel_type == ChannelType::Noise
            {
                Some(VolumeEnvelope::default())
            } else {
                None
            },
            timer: if channel_type == ChannelType::SquareWave
                || channel_type == ChannelType::WaveForm
            {
                Some(Timer::new(channel_type))
            } else {
                None
            },
            programmable_wave: if channel_type == ChannelType::WaveForm {
                Some(ProgrammableWave::default())
            } else {
                None
            },
            channel_type,
        }
    }

    pub fn step(&mut self) {
        if let Some(ref mut timer) = self.timer {
            let reached_zero = timer.step();
            if reached_zero {
                if let Some(ref mut duty) = self.duty {
                    duty.step();
                }
                if let Some(ref mut programmable_wave) = self.programmable_wave {
                    programmable_wave.step();
                }
            }
        }
    }

    pub fn frame_sequencer(&mut self, step: u8) {
        if step == 0 || step == 2 || step == 4 || step == 6 {
            self.length_counter_step();
        }
        if self.sweep.is_some() && (step == 2 || step == 6) {
            self.sweep_step();
        }
        if self.volume_envelope.is_some() && step == 7 {
            self.volume_envelope_step();
        }
    }

    pub fn length_counter_step(&mut self) {
        self.length_counter.step();
        self.enabled = !self.length_counter.should_disabled_channel();
    }

    pub fn volume_envelope_step(&mut self) {
        if let Some(ref mut ve) = self.volume_envelope {
            (*ve).step();
        }
    }

    pub fn sweep_step(&mut self) {
        if let Some(ref mut sweep) = self.sweep {
            let reached_zero = sweep.step();
            if reached_zero && sweep.enabled && sweep.period > 0 {
                let new_frequency = sweep.calculate_frequency();
                if sweep.is_overflowing(new_frequency) {
                    self.enabled = false;
                } else if sweep.shift_nb > 0 {
                    if let Some(ref mut timer) = self.timer {
                        (*timer).frequency = new_frequency;
                    }
                    sweep.shadow_frequency = new_frequency;
                    let new_frequency = sweep.calculate_frequency();
                    self.enabled = !sweep.is_overflowing(new_frequency);
                };
            }
        }
    }

    pub fn get_dac_output(&self) -> f32 {
        if !self.enabled {
            return 0.0;
        }
        let dac_input = if let Some(volume_envelope) = &self.volume_envelope {
            if let Some(duty) = &self.duty {
                (duty.get_amplitude() * volume_envelope.volume) as f32
            } else {
                0.0
            }
        } else if let Some(programmable_wave) = &self.programmable_wave {
            programmable_wave.get_dac_input()
        } else {
            0.0
        };

        (dac_input as f32 / 7.5) - 1.0
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
            Nr10 | Nr30 => {
                if let Some(sweep) = &self.sweep {
                    let mut res = 0;
                    res |= (sweep.period & 0x7) << 4;
                    res |= match sweep.direction {
                        Direction::Dec => 0b1000,
                        Direction::Inc => 0,
                    };
                    res |= sweep.shift_nb & 0x7;
                    return Ok(res);
                } else if self.channel_type == ChannelType::WaveForm {
                    return Ok(if self.enabled { 0x80 } else { 0 });
                }
                Ok(0)
            }
            Nr11 | Nr21 | Nr31 | Nr41 => {
                let mut res = 0;
                if let Some(duty) = &self.duty {
                    res = duty.pattern_index << 6;
                }
                res |= self.length_counter.length_load;
                Ok(res)
            }
            Nr12 | Nr22 | Nr32 | Nr42 => {
                if let Some(ve) = &self.volume_envelope {
                    let mut res = 0;
                    res |= ve.initial_volume << 4;
                    res |= match ve.envelope_direction {
                        Direction::Inc => 0b1000,
                        Direction::Dec => 0,
                    };
                    res |= ve.period & 0x7;
                    return Ok(res);
                }
                if let Some(pw) = &self.programmable_wave {
                    let res = match pw.volume_shift {
                        4 => 0b00, // mute
                        0 => 0b01, // 100%
                        1 => 0b10, // 50%
                        2 => 0b11, // 25%
                        _ => unreachable!(),
                    };
                    return Ok((res << 5) & 0b0110_0000);
                }
                Ok(0)
            }
            Nr13 | Nr23 | Nr33 | Nr43 => {
                if let Some(timer) = &self.timer {
                    if self.channel_type == ChannelType::Noise {
                        let mut res = 0;
                        res |= timer.shift_amout << 4;
                        res |= timer.divisor_code & 0x7;
                        return Ok(res);
                    } else {
                        return Ok(timer.frequency as u8);
                    }
                }
                Ok(0)
            }
            Nr14 | Nr24 | Nr34 | Nr44 => {
                let mut res = 0;
                res |= if self.enabled { 0x80 } else { 0 };
                res |= if self.length_counter.enabled { 0x40 } else { 0 };

                if self.channel_type == ChannelType::SquareWave
                    || self.channel_type == ChannelType::WaveForm
                {
                    if let Some(timer) = &self.timer {
                        res |= ((timer.frequency >> 8) & 0x07) as u8;
                    }
                }

                Ok(res)
            }
            WaveRam0 | WaveRam1 | WaveRam2 | WaveRam3 | WaveRam4 | WaveRam5 | WaveRam6
            | WaveRam7 | WaveRam8 | WaveRam9 | WaveRamA | WaveRamB | WaveRamC | WaveRamD
            | WaveRamE | WaveRamF => {
                let index = (u16::from(addr) - WAVE_RAM_0) * 2;

                Ok(self
                    .programmable_wave
                    .as_ref()
                    .unwrap()
                    .get_samples_at_index(index as usize))
            }

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
            Nr10 | Nr30 => {
                if let Some(ref mut sweep) = self.sweep {
                    (*sweep).period = (v >> 4) & 0x7;
                    (*sweep).direction = if v & 0b1000 != 0 {
                        Direction::Dec
                    } else {
                        Direction::Inc
                    };
                    (*sweep).shift_nb = v & 0x7;
                }
                if self.channel_type == ChannelType::WaveForm {
                    self.enabled = v & 0x80 != 0;
                }
            }
            Nr11 | Nr21 | Nr31 | Nr41 => {
                if self.channel_type == ChannelType::SquareWave {
                    if let Some(ref mut duty) = self.duty {
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
            Nr12 | Nr22 | Nr32 | Nr42 => {
                if let Some(ref mut ve) = self.volume_envelope {
                    (*ve).initial_volume = v >> 4;
                    (*ve).envelope_direction = if v & 0b1000 != 0 {
                        Direction::Inc
                    } else {
                        Direction::Dec
                    };
                    (*ve).period = v & 0x7;
                    self.enabled =
                        (*ve).initial_volume > 0 || (*ve).envelope_direction == Direction::Inc;
                }
                if let Some(ref mut pw) = self.programmable_wave {
                    (*pw).volume_shift = match (v & 0b0110_0000) >> 5 {
                        0b00 => 4, // mute
                        0b01 => 0, // 100%
                        0b10 => 1, // 50%
                        0b11 => 2, // 25%
                        _ => unreachable!(),
                    };
                }
            }
            Nr13 | Nr23 | Nr33 | Nr43 => {
                if let Some(ref mut timer) = self.timer {
                    if self.channel_type == ChannelType::Noise {
                        (*timer).shift_amout = v >> 4;
                        (*timer).divisor_code = v & 0x7;
                    } else {
                        let high_byte = (*timer).frequency.to_le_bytes()[1];
                        (*timer).frequency = (high_byte as u16 & 0x7) << 8 | v as u16;
                    }
                }
            }
            Nr14 | Nr24 | Nr34 | Nr44 => {
                self.enabled = v & 0x80 != 0;
                self.length_counter.enabled = v & 0x40 != 0;

                if self.channel_type == ChannelType::SquareWave
                    || self.channel_type == ChannelType::WaveForm
                {
                    if let Some(ref mut timer) = self.timer {
                        let low_byte = (*timer).frequency.to_le_bytes()[0];
                        (*timer).frequency = (v as u16 & 0x07) << 8 | low_byte as u16;
                    }
                }

                if self.enabled {
                    self.length_counter.reload();
                    if let Some(ref mut ve) = self.volume_envelope {
                        (*ve).reload();
                    }
                    if let Some(ref mut sweep) = self.sweep {
                        self.enabled = (*sweep).reload(self.timer.as_ref().unwrap().frequency);
                    }
                }
            }
            WaveRam0 | WaveRam1 | WaveRam2 | WaveRam3 | WaveRam4 | WaveRam5 | WaveRam6
            | WaveRam7 | WaveRam8 | WaveRam9 | WaveRamA | WaveRamB | WaveRamC | WaveRamD
            | WaveRamE | WaveRamF => {
                let index = (u16::from(addr) - WAVE_RAM_0) * 2;

                self.programmable_wave
                    .as_mut()
                    .unwrap()
                    .set_samples_at_index(index as usize, v);
            }

            _ => return Err(Error::SegmentationFault(addr.into())),
        };
        Ok(())
    }
}
