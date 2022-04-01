pub mod apu;
pub mod channels;
pub mod control;

#[derive(PartialEq, Clone, Copy)]
pub enum ChannelType {
    SquareWave,
    WaveForm,
    Noise,
}
