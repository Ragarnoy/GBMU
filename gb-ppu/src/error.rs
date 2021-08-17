use std::error::Error as STDError;
use std::fmt::{self, Debug, Display};

pub type PPUResult<T, E> = Result<T, Error<E>>;

#[derive(Debug)]
pub enum Error<T>
where
    T: Display + Debug,
{
    OutOfBound {
        value: T,
        min_bound: T,
        max_bound: T,
    },
}

impl<T> Display for Error<T>
where
    T: Display + Debug,
{
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

impl<T> STDError for Error<T> where T: Display + Debug {}
