pub mod apu;
pub mod channel;
pub mod control;

#[derive(PartialEq, Clone, Copy)]
pub enum ChannelType {
    SquareWave,
    WaveForm,
    Noise,
}
