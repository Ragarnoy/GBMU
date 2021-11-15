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

#[derive(Debug)]
pub struct Breakpoint {
    r#type: BreakpointType,
    pub enabled: bool,
    last_triggered: Option<usize>,
}

impl Default for Breakpoint {
    fn default() -> Self {
        Self {
            r#type: BreakpointType::default(),
            enabled: true,
            last_triggered: None,
        }
    }
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
            ..Default::default()
        }
    }

    pub fn address(&self) -> u16 {
        match self.r#type {
            BreakpointType::Address(x) => x,
        }
    }

    /// check if breakpoint is active
    /// this method is used to prevent the breakpoint to trigger itself on the same session
    pub fn active(&self, counter: usize) -> bool {
        self.enabled && self.last_triggered == Some(counter)
    }

    /// The method is used to register that the breakpoint was triggered at the session
    pub fn trigger(&mut self, counter: usize) {
        self.last_triggered = Some(counter);
    }
}
