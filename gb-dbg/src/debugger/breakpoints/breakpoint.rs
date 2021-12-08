use super::parser::Node;
use crate::dbg_interfaces::DebugOperations;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct Breakpoint {
    expr: Node,
    pub enabled: bool,
    last_state: bool,
}

impl Display for Breakpoint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl Breakpoint {
    pub fn from_address(address: u16) -> Self {
        Self {
            expr: Node::simple(address),
            enabled: true,
            last_state: false,
        }
    }

    pub fn from_expression(expr: &str) -> anyhow::Result<Self> {
        let node = Node::from_str(expr)?;
        Ok(Self {
            expr: node,
            enabled: true,
            last_state: false,
        })
    }

    pub fn is_triggered<T: DebugOperations>(&self, regs: &T) -> bool {
        self.enabled && false // self.expr.compute(regs)
    }

    /// check if breakpoint is active
    /// this method is used to prevent the breakpoint to trigger itself on the same session
    pub fn active<DBG: DebugOperations>(&mut self, context: &DBG) -> bool {
        let current_state = self.is_triggered(context);
        let result = !self.last_state && current_state;

        self.last_state = current_state;

        result
    }
}
