use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
/// The list of input supported by the gameboy.
pub enum InputType {
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    B,
    A,
}
