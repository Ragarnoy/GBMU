use anyhow::Result;
use std::fmt::{self, Display};

pub struct RegisterMap<T: Display>(pub T, pub RegisterValue);

#[derive(Clone, Copy)]
pub enum RegisterValue {
    U8(u8),
    U16(u16),
}

impl From<u8> for RegisterValue {
    fn from(input: u8) -> Self {
        Self::U8(input)
    }
}

impl From<u16> for RegisterValue {
    fn from(input: u16) -> Self {
        Self::U16(input)
    }
}

impl From<RegisterValue> for u16 {
    fn from(input: RegisterValue) -> Self {
        match input {
            RegisterValue::U8(x) => x as u16,
            RegisterValue::U16(x) => x,
        }
    }
}

pub trait DebugOperations: MemoryDebugOperations + RegisterDebugOperations {}

pub trait MemoryDebugOperations {
    fn read(&self, index: u16) -> u8;
}

pub trait RegisterDebugOperations {
    fn cpu_get(&self, key: CpuRegs) -> Result<RegisterValue>;

    fn ppu_get(&self, key: PpuRegs) -> Result<RegisterValue>;

    fn io_get(&self, key: IORegs) -> Result<RegisterValue>;

    fn cpu_registers(&self) -> Vec<RegisterMap<CpuRegs>>;

    fn ppu_registers(&self) -> Vec<RegisterMap<PpuRegs>>;

    fn io_registers(&self) -> Vec<RegisterMap<IORegs>>;
}

pub enum CpuRegs {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl Display for CpuRegs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            CpuRegs::AF => "AF",
            CpuRegs::BC => "BC",
            CpuRegs::DE => "DE",
            CpuRegs::HL => "HL",
            CpuRegs::SP => "SP",
            CpuRegs::PC => "PC",
        };
        write!(f, "{}", name)
    }
}

pub enum PpuRegs {
    Control,
    Status,
    Scy,
    Scx,
    Ly,
    Lyc,
    Dma,
    Bgp,
    Obp0,
    Obp1,
    Wy,
    Wx,
}

impl Display for PpuRegs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            PpuRegs::Control => "Control",
            PpuRegs::Status => "Status",
            PpuRegs::Scy => "Scy",
            PpuRegs::Scx => "Scx",
            PpuRegs::Ly => "Ly",
            PpuRegs::Lyc => "Lyc",
            PpuRegs::Dma => "Dma",
            PpuRegs::Bgp => "Bg p",
            PpuRegs::Obp0 => "Obj p0",
            PpuRegs::Obp1 => "Obj p1",
            PpuRegs::Wy => "Wy",
            PpuRegs::Wx => "Wx",
        };
        write!(f, "{}", name)
    }
}

pub enum IORegs {
    Joy,

    SerialByte,
    SerialControl,

    Div,
    Tima,
    Tma,
    Tac,

    If,
    Ie,

    BootRom,

    Fs1,
    Pwm1,
    Env1,
    Af1,
    Ctl1,

    Pwm2,
    Env2,
    Af2,
    Ctl2,

    A3Toggle,
    Pwm3,
    Vol3,
    Af3,
    Ctl3,

    Pwm4,
    Vol4,
    Af4,
    Ctl4,

    AudOutMap,
    AudMap,
    AudChanCtl,
    AudWave,
}

impl Display for IORegs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            IORegs::Joy => "Joypad",
            IORegs::SerialByte => "Serial Byte",
            IORegs::SerialControl => "Serial Control",
            IORegs::Div => "Div",
            IORegs::Tima => "Tima",
            IORegs::Tma => "Tma",
            IORegs::Tac => "TAc",
            IORegs::If => "Interrupt Flag",
            IORegs::Ie => "Interrupt Enable",
            IORegs::BootRom => "BootRom",
            IORegs::Fs1 => "Aud 1 Sweep",
            IORegs::Pwm1 => "Aud 1 Wave Duty",
            IORegs::Env1 => "Aud 1 Envelope",
            IORegs::Af1 => "Aud 1 Freq",
            IORegs::Ctl1 => "Aud 1 Ctl",
            IORegs::Pwm2 => "Aud 2 Wave Duty",
            IORegs::Env2 => "Aud 2 Envelope",
            IORegs::Af2 => "Aud 2 Freq",
            IORegs::Ctl2 => "Aud 2 Ctl",
            IORegs::A3Toggle => "Audio Channel 3 Toggle",
            IORegs::Pwm3 => "Aud 3 Wave Duty",
            IORegs::Vol3 => "Aud 3 Vol",
            IORegs::Af3 => "Aud 3 Freq",
            IORegs::Ctl3 => "Aud 3 Ctl",
            IORegs::Pwm4 => "Aud 4 Wave Duty",
            IORegs::Vol4 => "Aud 4 Vol",
            IORegs::Af4 => "Aud 4 Freq",
            IORegs::Ctl4 => "Aud 4 Ctl",
            IORegs::AudOutMap => "Aud Output Mapping",
            IORegs::AudMap => "Aud Mapping",
            IORegs::AudChanCtl => "Aud Channel Ctl",
            IORegs::AudWave => "Aud Wave",
        };
        write!(f, "{}", name)
    }
}
