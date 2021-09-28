
enum BreakpointType {
    Address(u16)
}

pub struct Breakpoint {
    r#type: BreakpointType,
    pub enabled: bool,
}

impl Breakpoint {
    pub fn from_address(address: u16) -> Self {
        Self {
            r#type: BreakpointType::Address(address),
            enabled: false,
        }
    }
    
    pub fn r#type(&self) -> String {
        match self.r#type {
            BreakpointType::Address(x) => format!("0x{:04X}", x)
        }
    }
}