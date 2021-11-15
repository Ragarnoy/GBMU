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
    bp_type: BreakpointType,
    pub enabled: bool,
}

impl Display for Breakpoint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.bp_type {
            BreakpointType::Address(x) => write!(f, "0x{:04X}", x),
        }
    }
}

impl Breakpoint {
    pub fn from_address(address: u16) -> Self {
        Self {
            bp_type: BreakpointType::Address(address),
            enabled: true,
        }
    }

    pub fn address(&self) -> u16 {
        match self.bp_type {
            BreakpointType::Address(x) => x,
        }
    }
}
