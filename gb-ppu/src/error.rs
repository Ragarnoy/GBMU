use std::error::Error as STDError;
use std::fmt::{self, Debug, Display};

pub type PPUResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    OutOfBound {
        value: usize,
        min_bound: usize,
        max_bound: usize,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfBound {
                value,
                min_bound,
                max_bound,
            } => write!(
                f,
                "value '{}' out bound, expected between '{}' and '{}'",
                value, min_bound, max_bound
            ),
        }
    }
}

impl STDError for Error {}
