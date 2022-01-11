use gb_breakpoint::Ast;
use std::{
    fmt::{self, Display},
    str::FromStr,
};

use crate::dbg_interfaces::DebugOperations;

use super::evaluation::is_expression_true;

#[derive(Debug)]
pub struct Breakpoint {
    expr: Ast,
    pub enabled: bool,
    last_state: bool,
}

impl Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.expr.to_string().split_whitespace().count() > 3 {
            let mut ret = String::with_capacity(self.expr.to_string().len() + 2);
            for (i, token) in self.expr.to_string().split_whitespace().enumerate() {
                if i != 0 {
                    if i % 4 == 0 {
                        ret.push('\n');
                    } else {
                        ret.push(' ');
                    }
                }
                ret.push_str(token);
            }
            write!(f, "{}", ret)
        } else {
            write!(f, "{}", self.expr)
        }
    }
}

impl Breakpoint {
    pub fn from_address(address: u16) -> Self {
        Self {
            expr: Ast::simple(address),
            enabled: true,
            last_state: false,
        }
    }

    pub fn from_expression(expr: &str) -> anyhow::Result<Self> {
        let node = Ast::from_str(expr).map_err(|e| anyhow::anyhow!(e))?;

        log::debug!("parsed expression: {:?}", node);
        Ok(Self {
            expr: node,
            enabled: true,
            last_state: false,
        })
    }

    pub fn is_triggered<T: DebugOperations>(&self, regs: &T) -> bool {
        self.enabled && is_expression_true(&self.expr, regs)
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
