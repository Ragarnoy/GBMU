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
pub const BUFFER_SIZE: usize = 2048;
pub const T_CYCLE_FREQUENCY: u32 = 0x40_0000;
