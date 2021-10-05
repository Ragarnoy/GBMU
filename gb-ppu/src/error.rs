use std::error::Error as STDError;
use std::fmt::{self, Debug, Display};

pub type PPUResult<T> = Result<T, PPUError>;

#[derive(Debug)]
pub enum PPUError {
    OutOfBound {
        value: usize,
        min_bound: usize,
        max_bound: usize,
    },
    MemoryUnavailable {
        mem_name: String,
    },
    RegistersUnavailable {
        reg_name: String,
    },
}

impl Display for PPUError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PPUError::OutOfBound {
                value,
                min_bound,
                max_bound,
            } => write!(
                f,
                "value '{}' out bound, expected between '{}' and '{}'",
                value, min_bound, max_bound
            ),
            PPUError::MemoryUnavailable { mem_name } => {
                write!(f, "memory '{}' is unavailable", mem_name)
            }
            PPUError::RegistersUnavailable { reg_name } => {
                write!(f, "registers '{}' are unavailable", reg_name)
            }
        }
    }
}

impl STDError for PPUError {}
