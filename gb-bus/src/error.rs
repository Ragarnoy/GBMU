#[derive(Debug, PartialEq)]
pub enum Error {
    BusError(u16),
    SegmentationFault(u16),
    InvalidIORegAddress(u16),
}

impl Error {
    pub fn bus_error(addr: u16) -> Self {
        Self::BusError(addr)
    }

    pub fn new_segfault(addr: u16) -> Self {
        Self::SegmentationFault(addr)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BusError(addr) => write!(f, "bus error at {:x}", addr),
            Error::SegmentationFault(addr) => write!(f, "segmentation fault at {:x}", addr),
            Error::InvalidIORegAddress(addr) => write!(f, "no io register for address {:x}", addr),
        }
    }
}

impl std::error::Error for Error {}
