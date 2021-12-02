use crate::InputType;
use gb_bus::Bus;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Mode {
    None = 0b0000_0000,
    Action = 0b0010_0000,
    Direction = 0b0001_0000,
    Both = 0b0011_0000,
}

impl Default for Mode {
    fn default() -> Self {
        Self::None
    }
}

impl From<u8> for Mode {
    fn from(v: u8) -> Self {
        match v {
            0b0010_0000 => Mode::Action,
            0b0001_0000 => Mode::Direction,
            0b0011_0000 => Mode::Both,
            _ => Mode::None,
        }
    }
}

pub fn register_from_state<'a, It>(mode: Mode, state: It) -> u8
where
    It: Iterator<Item = (&'a InputType, &'a bool)>,
{
    let mut v = mode as u8;
    if (v & Mode::Both as u8) != 0 {
        let direction_enabled = (v & Mode::Direction as u8) == Mode::Direction as u8;
        let action_enabled = (v & Mode::Action as u8) == Mode::Action as u8;
        for (ipt_type, toggle) in state {
            if !toggle {
                continue;
            }
            if action_enabled {
                match ipt_type {
                    InputType::A => v |= 0b0001,
                    InputType::B => v |= 0b0010,
                    InputType::Select => v |= 0b0100,
                    InputType::Start => v |= 0b1000,
                    _ => {}
                }
            }
            if direction_enabled {
                match ipt_type {
                    InputType::Right => v |= 0b0001,
                    InputType::Left => v |= 0b0010,
                    InputType::Up => v |= 0b0100,
                    InputType::Down => v |= 0b1000,
                    _ => {}
                }
            }
        }
    }
    !v
}

#[cfg(test)]
mod unit_reg_from_state {
    use super::{register_from_state, InputType, Mode};

    #[test]
    fn empty_state() {
        assert_eq!(register_from_state(Mode::None, vec![].into_iter()), 0xff);
        assert_eq!(
            register_from_state(Mode::Both, vec![].into_iter()),
            0b1100_1111
        );
        assert_eq!(
            register_from_state(Mode::Direction, vec![].into_iter()),
            0b1110_1111
        );
        assert_eq!(
            register_from_state(Mode::Action, vec![].into_iter()),
            0b1101_1111
        );
    }

    #[test]
    fn action_only() {
        let input = vec![(&InputType::A, &true), (&InputType::Select, &true)];

        assert_eq!(
            register_from_state(Mode::None, input.clone().into_iter()),
            0xff
        );
        assert_eq!(
            register_from_state(Mode::Direction, input.clone().into_iter()),
            !(Mode::Direction as u8)
        );
        assert_eq!(
            register_from_state(Mode::Action, input.clone().into_iter()),
            0b1101_1010
        );
    }

    #[test]
    fn direction_only() {
        let input = vec![(&InputType::Left, &true), (&InputType::Down, &true)];

        assert_eq!(
            register_from_state(Mode::None, input.clone().into_iter()),
            0xff
        );
        assert_eq!(
            register_from_state(Mode::Action, input.clone().into_iter()),
            !(Mode::Action as u8)
        );
        assert_eq!(
            register_from_state(Mode::Direction, input.clone().into_iter()),
            0b1110_0101
        );
    }

    #[test]
    fn both() {
        let input = vec![
            (&InputType::A, &true),
            (&InputType::Up, &true),
            (&InputType::Down, &true),
            (&InputType::Start, &true),
        ];

        assert_eq!(
            register_from_state(Mode::None, input.clone().into_iter()),
            0xff
        );
        assert_eq!(
            register_from_state(Mode::Action, input.clone().into_iter()),
            0b1101_0110
        );
        assert_eq!(
            register_from_state(Mode::Direction, input.clone().into_iter()),
            0b1110_0011
        );
        assert_eq!(
            register_from_state(Mode::Both, input.clone().into_iter()),
            0b1100_0010
        );
    }
}

pub fn trigger_interrupt(addr_bus: &mut dyn Bus<u8>) {
    const INTERRUPT_FLAG: u16 = 0xFF0F;
    const INTERRUPT_BIT: u8 = 0b10000;

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
