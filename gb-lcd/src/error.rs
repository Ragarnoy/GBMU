use std::fmt;

#[derive(Debug)]
pub enum Error {
    MainSys(String),
    GBWindowInit(String),
    GBWindowFrame(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MainSys(msg) => write!(f, "Failed to init global system: {}", msg),
            Error::GBWindowInit(msg) => write!(f, "Failed to init a GB Window: {}", msg),
            Error::GBWindowFrame(msg) => write!(f, "Error while runing a GB Window frame: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
