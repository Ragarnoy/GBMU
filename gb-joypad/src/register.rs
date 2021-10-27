use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2},
};

#[bitfield]
#[derive(Default, Debug, Clone, Copy)]
struct RegisterBits {
    p10: B1,
    p11: B1,
    p12: B1,
    p13: B1,
    p14: B1,
    p15: B1,
    #[allow(dead_code)]
    void: B2,
}

#[derive(Debug, Clone, Copy)]
enum RegisterMode {
    Direction,
    Action,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct JoypadRegister {
    bits: RegisterBits,
    mode: RegisterMode,
}

impl JoypadRegister {
    pub fn new() -> Self {
        JoypadRegister {
            bits: RegisterBits::new(),
            mode: RegisterMode::Direction,
        }
    }
}

impl Default for RegisterMode {
    fn default() -> RegisterMode {
        RegisterMode::Direction
    }
}

impl From<RegisterBits> for u8 {
    fn from(bits: RegisterBits) -> u8 {
        bits.into_bytes()[0]
    }
}

impl From<JoypadRegister> for u8 {
    fn from(register: JoypadRegister) -> u8 {
        register.bits.into()
    }
}

impl From<u8> for RegisterBits {
    fn from(byte: u8) -> RegisterBits {
        RegisterBits::from_bytes([byte])
    }
}

impl From<u8> for JoypadRegister {
    fn from(byte: u8) -> JoypadRegister {
        JoypadRegister {
            bits: byte.into(),
            mode: RegisterMode::Direction,
        }
    }
}
