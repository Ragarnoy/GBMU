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
