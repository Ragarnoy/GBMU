pub mod apu;
pub mod channel;
pub mod control;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChannelType {
    SquareWave,
    WaveForm,
    Noise,
}

pub const OUTPUT_CHANNELS_NB: usize = 2;
pub const SAMPLE_RATE: usize = 44100;
pub const SAMPLES_PER_FRAME: usize = SAMPLE_RATE / 60 * OUTPUT_CHANNELS_NB;

pub const MASK_UNUSED_BITS_FF: u8 = 0xFF;
pub const MASK_UNUSED_BITS_3F: u8 = 0x3F;
pub const MASK_UNUSED_BITS_7F: u8 = 0x7F;
pub const MASK_UNUSED_BITS_9F: u8 = 0x9F;
pub const MASK_UNUSED_BITS_BF: u8 = 0xBF;
pub const MASK_UNUSED_BITS_70: u8 = 0x70;
pub const MASK_UNUSED_BITS_80: u8 = 0x80;