use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum BreakpointType {
    Address(u16),
}

impl Default for BreakpointType {
    fn default() -> Self {
        Self::Address(0)
    }
}

#[derive(Default, Debug)]
pub struct Breakpoint {
    r#type: BreakpointType,
    pub enabled: bool,
}

impl Display for Breakpoint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.r#type {
            BreakpointType::Address(x) => write!(f, "0x{:04X}", x),
        }
    }
}

impl Breakpoint {
    pub fn from_address(address: u16) -> Self {
        Self {
            r#type: BreakpointType::Address(address),
            enabled: false,
        }
    }

    pub fn address(&self) -> u16 {
        match self.r#type {
            BreakpointType::Address(x) => x,
        }
    }
}
