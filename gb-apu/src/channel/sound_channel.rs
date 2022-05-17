use crate::{
    channel::duty::Duty,
    channel::length_counter::LengthCounter,
    channel::lfsr::{Lfsr, WidthMode},
    channel::sweep::Sweep,
    channel::timer::Timer,
    channel::volume_envelope::{Direction, VolumeEnvelope},
    ChannelType, MASK_UNUSED_BITS_3F, MASK_UNUSED_BITS_7F, MASK_UNUSED_BITS_80,
    MASK_UNUSED_BITS_9F, MASK_UNUSED_BITS_BF, MASK_UNUSED_BITS_FF,
};
use gb_bus::{io_reg_constant::WAVE_RAM_0, Address, Error, FileOperation, IORegArea, Source};

use super::wave_ram::ProgrammableWave;

#[derive(Debug)]
pub struct SoundChannel {
    pub enabled: bool,
    pub dac_enabled: bool,
    trigger: bool,
    pub channel_type: ChannelType,
    sweep: Option<Sweep>,
    duty: Option<Duty>,
    length_counter: LengthCounter,
    volume_envelope: Option<VolumeEnvelope>,
    timer: Timer,
    programmable_wave: Option<ProgrammableWave>,
    lfsr: Option<Lfsr>,
}

impl SoundChannel {
    pub fn new(channel_type: ChannelType, handles_sweep: bool) -> Self {
        SoundChannel {
            enabled: false,
            dac_enabled: false,
            trigger: false,
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
            timer: Timer::new(channel_type),
            programmable_wave: if channel_type == ChannelType::WaveForm {
                Some(ProgrammableWave::default())
            } else {
                None
            },
            lfsr: if channel_type == ChannelType::Noise {
                Some(Lfsr::default())
            } else {
                None
            },
            channel_type,
        }
    }

    pub fn reset(&mut self) -> Self {
        if let Some(pw) = &mut self.programmable_wave {
            (*pw).bits = 0;
        }
        SoundChannel {
            enabled: false,
            dac_enabled: false,
            trigger: false,
            sweep: if self.sweep.is_some() && self.channel_type == ChannelType::SquareWave {
                Some(Sweep::default())
            } else {
                None
            },
            duty: if self.channel_type == ChannelType::SquareWave {
                Some(Duty::default())
            } else {
                None
            },
            length_counter: LengthCounter::new(self.channel_type),
            volume_envelope: if self.channel_type == ChannelType::SquareWave
                || self.channel_type == ChannelType::Noise
            {
                Some(VolumeEnvelope::default())
            } else {
                None
            },
            timer: Timer::new(self.channel_type),
            programmable_wave: self.programmable_wave.clone(),
            lfsr: if self.channel_type == ChannelType::Noise {
                Some(Lfsr::default())
            } else {
                None
            },
            channel_type: self.channel_type,
        }
    }

    pub fn step(&mut self) {
        let reached_zero = self.timer.step();
        if reached_zero {
            if let Some(ref mut duty) = self.duty {
                duty.step();
            } else if let Some(ref mut lfsr) = self.lfsr {
                lfsr.step()
            } else if let Some(ref mut programmable_wave) = self.programmable_wave {
                programmable_wave.step();
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
                    self.timer.frequency = new_frequency;
                    sweep.shadow_frequency = new_frequency;
                    let new_frequency = sweep.calculate_frequency();
                    self.enabled = !sweep.is_overflowing(new_frequency);
                };
            }
        }
    }

    pub fn get_dac_output(&self) -> f32 {
        if !self.dac_enabled || !self.enabled {
            return 0.0;
        }
        let dac_input = if let Some(volume_envelope) = &self.volume_envelope {
            if let Some(duty) = &self.duty {
                (duty.get_amplitude() * volume_envelope.volume) as f32
            } else if let Some(ref lfsr) = self.lfsr {
                (lfsr.get_amplitude() * volume_envelope.volume) as f32
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
    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
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
                    return Ok(res | MASK_UNUSED_BITS_80);
                } else if self.channel_type == ChannelType::WaveForm {
                    return Ok(if self.dac_enabled { 0x80 } else { 0 } | MASK_UNUSED_BITS_7F);
                }
                Ok(0)
            }
            Nr11 | Nr21 | Nr31 | Nr41 => {
                if let Some(duty) = &self.duty {
                    let mut res = duty.pattern_index << 6;
                    res |= self.length_counter.length_load;
                    return Ok(res | MASK_UNUSED_BITS_3F);
                }
                Ok(MASK_UNUSED_BITS_FF)
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
                    return Ok(pw.bits | MASK_UNUSED_BITS_9F);
                }
                Ok(0)
            }
            Nr13 | Nr23 | Nr33 | Nr43 => {
                if self.channel_type == ChannelType::Noise {
                    let mut res = 0;
                    res |= self.timer.shift_amout << 4;
                    res |= match self.lfsr.as_ref().unwrap().width_mode {
                        WidthMode::Width7Bits => 0b1000,
                        WidthMode::Width14Bits => 0,
                    };
                    res |= self.timer.divisor_code & 0x7;
                    Ok(res)
                } else {
                    Ok(MASK_UNUSED_BITS_FF)
                }
            }
            Nr14 | Nr24 | Nr34 | Nr44 => {
                let mut res = 0;
                res |= if self.trigger { 0x80 } else { 0 };
                res |= if self.length_counter.enabled { 0x40 } else { 0 };

                if self.channel_type == ChannelType::SquareWave
                    || self.channel_type == ChannelType::WaveForm
                {
                    res |= ((self.timer.frequency >> 8) & 0x07) as u8;
                }

                Ok(res | MASK_UNUSED_BITS_BF)
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

    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
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
                    self.dac_enabled = v & 0x80 != 0;
                    if !self.dac_enabled {
                        self.enabled = false;
                    }
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
                    self.dac_enabled =
                        (*ve).initial_volume > 0 || (*ve).envelope_direction == Direction::Inc;
                    if !self.dac_enabled {
                        self.enabled = false;
                    }
                }
                if let Some(ref mut pw) = self.programmable_wave {
                    (*pw).bits = v;
                }
            }
            Nr13 | Nr23 | Nr33 | Nr43 => {
                if self.channel_type == ChannelType::Noise {
                    self.timer.shift_amout = v >> 4;
                    self.lfsr.as_mut().unwrap().width_mode = if v & 0b1000 != 0 {
                        WidthMode::Width7Bits
                    } else {
                        WidthMode::Width14Bits
                    };
                    self.timer.divisor_code = v & 0x7;
                } else {
                    let high_byte = self.timer.frequency.to_le_bytes()[1];
                    self.timer.frequency = (high_byte as u16 & 0x7) << 8 | v as u16;
                }
            }
            Nr14 | Nr24 | Nr34 | Nr44 => {
                self.trigger = v & 0x80 != 0;
                self.length_counter.enabled = v & 0x40 != 0;

                if self.channel_type == ChannelType::SquareWave
                    || self.channel_type == ChannelType::WaveForm
                {
                    let low_byte = self.timer.frequency.to_le_bytes()[0];
                    self.timer.frequency = (v as u16 & 0x07) << 8 | low_byte as u16;
                }

                if self.trigger {
                    self.length_counter.reload();
                    if let Some(ref mut ve) = self.volume_envelope {
                        (*ve).reload();
                    }
                    if let Some(ref mut sweep) = self.sweep {
                        self.enabled = (*sweep).reload(self.timer.frequency);
                    }
                    if let Some(ref mut lfsr) = self.lfsr {
                        (*lfsr).reload();
                    }
                    if self.dac_enabled {
                        self.enabled = true;
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
