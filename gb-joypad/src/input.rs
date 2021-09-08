use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
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
