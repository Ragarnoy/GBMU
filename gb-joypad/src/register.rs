use crate::InputType;
use gb_bus::Bus;
use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2},
};
use std::collections::HashMap;

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
    Unset,
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
            mode: RegisterMode::Unset,
        }
    }

    pub fn refresh(&mut self, addr_bus: &mut dyn Bus<u8>, state: &mut HashMap<InputType, bool>) {
        match self.mode {
            RegisterMode::Unset => {}
            RegisterMode::Direction => {
                self.bits.set_p10(state[&InputType::Right].into());
                self.bits.set_p11(state[&InputType::Left].into());
                self.bits.set_p12(state[&InputType::Up].into());
                self.bits.set_p13(state[&InputType::Down].into());
            }
            RegisterMode::Action => {
                self.bits.set_p10(state[&InputType::A].into());
                self.bits.set_p11(state[&InputType::B].into());
                self.bits.set_p12(state[&InputType::Select].into());
                self.bits.set_p13(state[&InputType::Start].into());
            }
        }
    }
}

impl Default for RegisterMode {
    fn default() -> RegisterMode {
        RegisterMode::Unset
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
        let bits: RegisterBits = byte.into();
        let mode = match (bits.p14(), bits.p15()) {
            (n, _) if n != 0 => RegisterMode::Direction,
            (_, n) if n != 0 => RegisterMode::Action,
            _ => RegisterMode::Unset,
        };
        JoypadRegister { bits, mode }
    }
}
