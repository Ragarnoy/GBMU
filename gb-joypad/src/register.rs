use crate::InputType;
use gb_bus::Bus;
use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2},
};
use std::collections::HashMap;

const INTERRUPT_FLAG: u16 = 0xFF0F;
const INTERRUPT_BIT: u8 = 0b10000;

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

impl RegisterBits {
    fn update(
        &mut self,
        values: [InputType; 4],
        state: &mut HashMap<InputType, bool>,
        addr_bus: &mut dyn Bus<u8>,
    ) {
        let old = self.p10();
        self.set_p10((!state[&values[0]]).into());
        if old != 0 && self.p10() == 0 {
            trigger_interrupt(addr_bus);
        };

        let old = self.p11();
        self.set_p11((!state[&values[1]]).into());
        if old != 0 && self.p11() == 0 {
            trigger_interrupt(addr_bus);
        };

        let old = self.p12();
        self.set_p12((!state[&values[2]]).into());
        if old != 0 && self.p12() == 0 {
            trigger_interrupt(addr_bus);
        };

        let old = self.p13();
        self.set_p13((!state[&values[3]]).into());
        if old != 0 && self.p13() == 0 {
            trigger_interrupt(addr_bus);
        };
    }
}

fn trigger_interrupt(addr_bus: &mut dyn Bus<u8>) {
    let interrupts_val = addr_bus
        .read(INTERRUPT_FLAG, None)
        .expect("Failed to read interrupt value for joypad interrupt");
    if let Err(err) = addr_bus.write(INTERRUPT_FLAG, interrupts_val | INTERRUPT_BIT, None) {
        log::error!(
            "Failed to write interrupt value for joypad interrupt: {:?}",
            err
        )
    }
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
                self.bits.update(
                    [
                        InputType::Right,
                        InputType::Left,
                        InputType::Up,
                        InputType::Down,
                    ],
                    state,
                    addr_bus,
                );
            }
            RegisterMode::Action => {
                self.bits.update(
                    [
                        InputType::A,
                        InputType::B,
                        InputType::Select,
                        InputType::Start,
                    ],
                    state,
                    addr_bus,
                );
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
