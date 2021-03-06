use std::fmt::{self, Debug, Display, Formatter};

use gb_breakpoint::register::Register;

pub struct RegisterMap<T: Display + Debug>(pub T, pub RegisterValue);

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

pub trait DebugOperations: MemoryDebugOperations + RegisterDebugOperations {
    fn cycle(&self) -> usize;
}

pub trait MemoryDebugOperations {
    fn read(&self, index: u16) -> u8;
}

pub trait RegisterDebugOperations {
    fn cpu_get(&self, key: CpuRegs) -> RegisterValue;

    fn ppu_get(&self, key: PpuRegs) -> RegisterValue;

    fn io_get(&self, key: IORegs) -> RegisterValue;

    fn audio_get(&self, key: AudioRegs) -> RegisterValue;

    fn cpu_registers(&self) -> Vec<RegisterMap<CpuRegs>>;

    fn ppu_registers(&self) -> Vec<RegisterMap<PpuRegs>>;

    fn io_registers(&self) -> Vec<RegisterMap<IORegs>>;

    fn audio_registers(&self) -> Vec<RegisterMap<AudioRegs>>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

impl TryFrom<Register> for CpuRegs {
    type Error = String;

    fn try_from(s: Register) -> Result<Self, Self::Error> {
        match s {
            Register::AF => Ok(CpuRegs::AF),
            Register::BC => Ok(CpuRegs::BC),
            Register::DE => Ok(CpuRegs::DE),
            Register::HL => Ok(CpuRegs::HL),
            Register::SP => Ok(CpuRegs::SP),
            Register::PC => Ok(CpuRegs::PC),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum IORegs {
    Joy,

    SerialByte,
    SerialCtl,

    Div,
    Tima,
    Tma,
    Tac,

    If,
    Ie,

    BootRom,

    Key1,
    VramBank,
    Hdma1,
    Hdma2,
    Hdma3,
    Hdma4,
    Hdma5,
    WRamBank,
}

impl Display for IORegs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            IORegs::Joy => "Joypad",
            IORegs::SerialByte => "Serial Byte",
            IORegs::SerialCtl => "Serial Control",
            IORegs::Div => "Div",
            IORegs::Tima => "Tima",
            IORegs::Tma => "Tma",
            IORegs::Tac => "Tac",
            IORegs::If => "Interrupt Flag",
            IORegs::Ie => "Interrupt Enable",
            IORegs::BootRom => "BootRom",
            IORegs::Key1 => "DSpeed",
            IORegs::VramBank => "VBank",
            IORegs::Hdma1 => "Hdma1",
            IORegs::Hdma2 => "Hdma2",
            IORegs::Hdma3 => "Hdma3",
            IORegs::Hdma4 => "Hdma4",
            IORegs::Hdma5 => "Hdma5",
            IORegs::WRamBank => "WBank",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug)]
pub enum AudioRegs {
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

    AudMap,
    AudChanCtl,
    AudWave,
}

impl Display for AudioRegs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            AudioRegs::Fs1 => "Audio 1 Sweep",
            AudioRegs::Pwm1 => "Audio 1 Wave Duty",
            AudioRegs::Env1 => "Audio 1 Envelope",
            AudioRegs::Af1 => "Audio 1 Freq",
            AudioRegs::Ctl1 => "Audio 1 Ctl",
            AudioRegs::Pwm2 => "Audio 2 Wave Duty",
            AudioRegs::Env2 => "Audio 2 Envelope",
            AudioRegs::Af2 => "Audio 2 Freq",
            AudioRegs::Ctl2 => "Audio 2 Ctl",
            AudioRegs::A3Toggle => "Audio Channel 3 Toggle",
            AudioRegs::Pwm3 => "Audio 3 Wave Duty",
            AudioRegs::Vol3 => "Audio 3 Vol",
            AudioRegs::Af3 => "Audio 3 Freq",
            AudioRegs::Ctl3 => "Audio 3 Ctl",
            AudioRegs::Pwm4 => "Audio 4 Wave Duty",
            AudioRegs::Vol4 => "Audio 4 Vol",
            AudioRegs::Af4 => "Audio 4 Freq",
            AudioRegs::Ctl4 => "Audio 4 Ctl",
            AudioRegs::AudMap => "Audio Mapping",
            AudioRegs::AudChanCtl => "Audio Channel Ctl",
            AudioRegs::AudWave => "Audio Wave",
        };
        write!(f, "{}", name)
    }
}
