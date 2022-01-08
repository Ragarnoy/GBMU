use std::error::Error;
use std::fmt::Display;

use super::{mbc1, mbc2, mbc3, mbc5};

pub trait SaveState {
    fn serialize(&self) -> Full;
    fn serialize_partial(&self) -> Partial {
        Partial::None
    }
    fn load(&self, state: Full) -> Result<(), StateError>;
    fn load_partial(&self, _state: Partial) -> Result<(), StateError> {
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Full {
    None,
    Mbc1(mbc1::Full),
    Mbc2(mbc2::Full),
    Mbc3(mbc3::Full),
    Mbc5(mbc5::Full),
}

impl Full {
    fn id(&self) -> String {
        match self {
            Full::None => "none",
            Full::Mbc1(_) => "mbc1",
            Full::Mbc2(_) => "mbc2",
            Full::Mbc3(_) => "mbc3",
            Full::Mbc5(_) => "mbc5",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Partial {
    None,
    Mbc2(mbc2::Partial),
    Mbc3(mbc3::Partial),
}

impl Partial {
    fn id(&self) -> String {
        match self {
            Partial::None => "none",
            Partial::Mbc2(_) => "mbc2",
            Partial::Mbc3(_) => "mbc3",
        }
    }
}

#[derive(Debug)]
pub enum StateError {
    WrongType { expected: String, got: String },
    RamLength { expected: usize, got: usize },
}

impl Error for StateError {}

impl Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::WrongType { expected, got } => {
                write!(f, "wrong state type: expected {} but got {}", expected, got)
            }
            StateError::RamLength { expected, got } => {
                write!(f, "ram length error: expected {} but got {}", expected, got)
            }
        }
    }
}
