pub mod apu;
pub mod channel;
pub mod control;

#[derive(PartialEq, Clone, Copy)]
pub enum ChannelType {
    SquareWave,
    WaveForm,
    Noise,
}

pub const BUFFER_SIZE: usize = 2048;