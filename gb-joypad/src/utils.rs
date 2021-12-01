use crate::InputType;

#[repr(u8)]
pub enum Mode {
    Action = 0b0010_0000,
    Direction = 0b0001_0000,
    Both = 0b0011_0000,
}

pub fn register_from_state<It>(mode: Option<Mode>, state: It) -> u8
where
    It: Iterator<Item = (InputType, bool)>,
{
    let mut v = mode.map(|m| m as u8).unwrap_or(0);
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
        assert_eq!(register_from_state(None, vec![].into_iter()), 0xff);
        assert_eq!(
            register_from_state(Some(Mode::Both), vec![].into_iter()),
            0b1100_1111
        );
        assert_eq!(
            register_from_state(Some(Mode::Direction), vec![].into_iter()),
            0b1110_1111
        );
        assert_eq!(
            register_from_state(Some(Mode::Action), vec![].into_iter()),
            0b1101_1111
        );
    }

    #[test]
    fn action_only() {
        let input = vec![(InputType::A, true), (InputType::Select, true)];

        assert_eq!(register_from_state(None, input.clone().into_iter()), 0xff);
        assert_eq!(
            register_from_state(Some(Mode::Direction), input.clone().into_iter()),
            !(Mode::Direction as u8)
        );
        assert_eq!(
            register_from_state(Some(Mode::Action), input.clone().into_iter()),
            0b1101_1010
        );
    }

    #[test]
    fn direction_only() {
        let input = vec![(InputType::Left, true), (InputType::Down, true)];

        assert_eq!(register_from_state(None, input.clone().into_iter()), 0xff);
        assert_eq!(
            register_from_state(Some(Mode::Action), input.clone().into_iter()),
            !(Mode::Action as u8)
        );
        assert_eq!(
            register_from_state(Some(Mode::Direction), input.clone().into_iter()),
            0b1110_0101
        );
    }

    #[test]
    fn both() {
        let input = vec![
            (InputType::A, true),
            (InputType::Up, true),
            (InputType::Down, true),
            (InputType::Start, true),
        ];

        assert_eq!(register_from_state(None, input.clone().into_iter()), 0xff);
        assert_eq!(
            register_from_state(Some(Mode::Action), input.clone().into_iter()),
            0b1101_0110
        );
        assert_eq!(
            register_from_state(Some(Mode::Direction), input.clone().into_iter()),
            0b1110_0011
        );
        assert_eq!(
            register_from_state(Some(Mode::Both), input.clone().into_iter()),
            0b1100_0010
        );
    }
}
