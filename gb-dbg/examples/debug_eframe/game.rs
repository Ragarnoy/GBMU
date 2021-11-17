use gb_dbg::dbg_interfaces::{
    AudioRegs, CpuRegs, DebugOperations, IORegs, MemoryDebugOperations, PpuRegs,
    RegisterDebugOperations, RegisterMap, RegisterValue,
};

pub struct Iter<'a> {
    count: u32,
    registers: &'a Game,
}

impl<'a> Iterator for Iter<'a> {
    type Item = RegisterMap<CpuRegs>;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        match self.count {
            1 => Some(RegisterMap(
                CpuRegs::AF,
                RegisterValue::from(self.registers.a),
            )),
            2 => Some(RegisterMap(
                CpuRegs::BC,
                RegisterValue::from(self.registers.b),
            )),
            3 => Some(RegisterMap(
                CpuRegs::DE,
                RegisterValue::from(self.registers.c),
            )),
            4 => Some(RegisterMap(
                CpuRegs::HL,
                RegisterValue::from(self.registers.d),
            )),
            5 => Some(RegisterMap(
                CpuRegs::SP,
                RegisterValue::from(self.registers.e),
            )),
            6 => Some(RegisterMap(
                CpuRegs::PC,
                RegisterValue::from(self.registers.pc),
            )),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub pc: u16,
    pub memory: Vec<u8>,
}

impl DebugOperations for Game {}

impl MemoryDebugOperations for Game {
    fn read(&self, index: u16) -> u8 {
        *self.memory.get(index as usize).unwrap()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            a: 0xFF,
            b: 0x1F,
            c: 0x22,
            d: 0x3F,
            e: 4,
            f: 8,
            pc: 500,
            memory: vec![0xFFu8; u16::MAX as usize],
        }
    }
}

impl Game {
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            count: 0,
            registers: self,
        }
    }
}

//TODO Temporary for now it looks like ass
impl From<&Game> for Vec<RegisterMap<CpuRegs>> {
    fn from(registers: &Game) -> Self {
        vec![
            RegisterMap(CpuRegs::AF, RegisterValue::from(registers.a)),
            RegisterMap(CpuRegs::BC, RegisterValue::from(registers.b)),
            RegisterMap(CpuRegs::DE, RegisterValue::from(registers.d)),
            RegisterMap(CpuRegs::HL, RegisterValue::from(registers.f)),
            RegisterMap(CpuRegs::PC, RegisterValue::from(registers.pc)),
        ]
    }
}

impl RegisterDebugOperations for Game {
    fn cpu_get(&self, key: CpuRegs) -> RegisterValue {
        match key {
            CpuRegs::AF => RegisterValue::from(self.a),
            CpuRegs::BC => RegisterValue::from(self.b),
            CpuRegs::DE => RegisterValue::from(self.d),
            CpuRegs::HL => RegisterValue::from(self.f),
            CpuRegs::SP => RegisterValue::from(self.c),
            CpuRegs::PC => RegisterValue::from(self.pc),
        }
    }

    fn ppu_get(&self, _key: PpuRegs) -> RegisterValue {
        0xffu8.into()
    }

    fn io_get(&self, _key: IORegs) -> RegisterValue {
        0xffu8.into()
    }

    fn audio_get(&self, _key: AudioRegs) -> RegisterValue {
        0xffu8.into()
    }

    fn cpu_registers(&self) -> Vec<RegisterMap<CpuRegs>> {
        self.into()
    }

    fn ppu_registers(&self) -> Vec<RegisterMap<PpuRegs>> {
        vec![
            RegisterMap(PpuRegs::Control, RegisterValue::from(2u8)),
            RegisterMap(PpuRegs::Status, RegisterValue::from(14u8)),
            RegisterMap(PpuRegs::Scy, RegisterValue::from(34u8)),
            RegisterMap(PpuRegs::Scx, RegisterValue::from(125u16)),
            RegisterMap(PpuRegs::Ly, RegisterValue::from(28u8)),
            RegisterMap(PpuRegs::Lyc, RegisterValue::from(556u16)),
            RegisterMap(PpuRegs::Dma, RegisterValue::from(444u16)),
            RegisterMap(PpuRegs::Bgp, RegisterValue::from(215u8)),
            RegisterMap(PpuRegs::Obp0, RegisterValue::from(33u8)),
            RegisterMap(PpuRegs::Obp1, RegisterValue::from(8u8)),
            RegisterMap(PpuRegs::Wy, RegisterValue::from(6u8)),
            RegisterMap(PpuRegs::Wx, RegisterValue::from(88u8)),
        ]
    }

    fn io_registers(&self) -> Vec<RegisterMap<IORegs>> {
        vec![
            RegisterMap(IORegs::Joy, RegisterValue::from(2u8)),
            RegisterMap(IORegs::SerialByte, RegisterValue::from(14u8)),
            RegisterMap(IORegs::SerialCtl, RegisterValue::from(34u8)),
            RegisterMap(IORegs::Div, RegisterValue::from(125u16)),
            RegisterMap(IORegs::Tima, RegisterValue::from(28u8)),
            RegisterMap(IORegs::Tma, RegisterValue::from(556u16)),
            RegisterMap(IORegs::Tac, RegisterValue::from(444u16)),
            RegisterMap(IORegs::If, RegisterValue::from(215u8)),
            RegisterMap(IORegs::Ie, RegisterValue::from(33u8)),
            RegisterMap(IORegs::BootRom, RegisterValue::from(81u16)),
        ]
    }

    fn audio_registers(&self) -> Vec<RegisterMap<AudioRegs>> {
        vec![
            RegisterMap(AudioRegs::Fs1, RegisterValue::from(22u8)),
            RegisterMap(AudioRegs::Pwm1, RegisterValue::from(2u8)),
            RegisterMap(AudioRegs::Env1, RegisterValue::from(14u8)),
            RegisterMap(AudioRegs::Af1, RegisterValue::from(34u8)),
            RegisterMap(AudioRegs::Ctl1, RegisterValue::from(125u16)),
            RegisterMap(AudioRegs::Pwm2, RegisterValue::from(288u16)),
            RegisterMap(AudioRegs::Env2, RegisterValue::from(555u16)),
            RegisterMap(AudioRegs::Af2, RegisterValue::from(444u16)),
            RegisterMap(AudioRegs::Ctl2, RegisterValue::from(215u8)),
            RegisterMap(AudioRegs::A3Toggle, RegisterValue::from(33u8)),
            RegisterMap(AudioRegs::Pwm3, RegisterValue::from(81u16)),
            RegisterMap(AudioRegs::Vol3, RegisterValue::from(48u16)),
            RegisterMap(AudioRegs::Af3, RegisterValue::from(8u16)),
            RegisterMap(AudioRegs::Ctl3, RegisterValue::from(99u16)),
            RegisterMap(AudioRegs::Pwm4, RegisterValue::from(269u16)),
            RegisterMap(AudioRegs::Vol4, RegisterValue::from(8654u16)),
            RegisterMap(AudioRegs::Af4, RegisterValue::from(69u16)),
            RegisterMap(AudioRegs::Ctl4, RegisterValue::from(6u16)),
            RegisterMap(AudioRegs::AudOutMap, RegisterValue::from(15042u16)),
            RegisterMap(AudioRegs::AudMap, RegisterValue::from(5555u16)),
            RegisterMap(AudioRegs::AudChanCtl, RegisterValue::from(251u8)),
            RegisterMap(AudioRegs::AudWave, RegisterValue::from(12u8)),
        ]
    }
}
