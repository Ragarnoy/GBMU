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
