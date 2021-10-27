use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2},
};

#[bitfield]
#[derive(Default, Debug)]
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

#[derive(Debug)]
enum RegisterMode {
    Direction,
    Action,
}

#[derive(Debug)]
pub struct JoypadRegister {
    bits: RegisterBits,
    mode: RegisterMode,
}

impl JoypadRegister {
    pub fn new() -> Self {
        JoypadRegister {
            bits: RegisterBits::default(),
            mode: RegisterMode::Direction,
        }
    }
}

impl Default for JoypadRegister {
    fn default() -> JoypadRegister {
        JoypadRegister::new()
    }
}
