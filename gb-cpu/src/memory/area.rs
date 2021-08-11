#[derive(Debug, PartialEq, Eq)]
pub enum Area {
    Rom,
    Vram,
    ExtRam,
    Wram,
    EchoRam,
    Oam,
    IOReg,
    HighRam,
    IEReg,
}
pub mod wram;
pub mod rom;
