use crate::dbg_interfaces::RegisterDebugOperations;
use crate::debugger::breakpoints::breakpoint_node::BreakpointNode;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct Breakpoint {
    expr: BreakpointNode,
    pub enabled: bool,
    last_triggered: Option<usize>,
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
            last_triggered: None,
        }
    }

    pub fn from_expression(expr: &str) -> anyhow::Result<Self> {
        let node = BreakpointNode::from_str(expr)?;
        Ok(Self {
            expr: node,
            enabled: true,
            last_triggered: None,
        })
    }

    pub fn is_triggered<T: RegisterDebugOperations>(&self, regs: &T) -> bool {
        if self.enabled {
            self.expr.compute(regs)
        } else {
            false
        }
    }

    pub fn is_triggered<DBG: DebugOperations>(&mut self, context: &DBG) -> bool {
        if self.active() {
            true
        }
        false
    }

    /// check if breakpoint is active
    /// this method is used to prevent the breakpoint to trigger itself on the same session
    pub fn active(&self, counter: usize) -> bool {
        self.enabled && self.last_triggered != Some(counter)
    }

    /// The method is used to register that the breakpoint was triggered at the session
    pub fn trigger(&mut self, counter: usize) {
        self.last_triggered = Some(counter);
    }
}
