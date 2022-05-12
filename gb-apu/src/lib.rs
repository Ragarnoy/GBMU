pub mod apu;
pub mod channel;
pub mod control;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChannelType {
    SquareWave,
    WaveForm,
    Noise,
}

pub const BUFFER_SIZE: usize = 2048;
pub const T_CYCLE_FREQUENCY: u32 = 0x40_0000;
