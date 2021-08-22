#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Area {
    Bios,
    Rom,
    Vram,
    ExtRam,
    Ram,
    ERam,
    Oam,
    IoReg,
    HighRam,
    IEReg,
    Unbound,
}
