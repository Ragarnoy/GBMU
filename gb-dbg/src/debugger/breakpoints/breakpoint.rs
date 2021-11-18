use crate::dbg_interfaces::RegisterDebugOperations;
use crate::debugger::breakpoints::breakpoint_node::BreakpointNode;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct Breakpoint {
    expr: BreakpointNode,
    pub enabled: bool,
}

impl Display for Breakpoint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl Breakpoint {
    pub fn from_address(address: u16) -> Self {
        Self {
            expr: BreakpointNode::new_simple(address),
            enabled: true,
        }
    }

    pub fn from_expression(expr: &str) -> anyhow::Result<Self> {
        let node = BreakpointNode::from_str(expr)?;
        Ok(Self {
            expr: node,
            enabled: true,
        })
    }

    pub fn is_triggered<T: RegisterDebugOperations>(&self, regs: &T) -> bool {
        if self.enabled {
            self.expr.compute(regs)
        } else {
            false
        }
    }
}
