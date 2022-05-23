use cpal::SampleRate;

pub mod apu;
pub mod channel;
pub mod control;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChannelType {
    SquareWave,
    WaveForm,
    Noise,
}

pub const SAMPLE_RATES: [SampleRate; 2] = [SampleRate(44100), SampleRate(48000)];

pub const MASK_UNUSED_BITS_FF: u8 = 0xFF;
pub const MASK_UNUSED_BITS_3F: u8 = 0x3F;
pub const MASK_UNUSED_BITS_7F: u8 = 0x7F;
pub const MASK_UNUSED_BITS_9F: u8 = 0x9F;
pub const MASK_UNUSED_BITS_BF: u8 = 0xBF;
pub const MASK_UNUSED_BITS_70: u8 = 0x70;
pub const MASK_UNUSED_BITS_80: u8 = 0x80;
pub const T_CYCLE_FREQUENCY: u32 = 0x40_0000;
pub const NB_CYCLES_512_HZ: u32 = 0x2000;
