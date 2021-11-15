enum Operator {
    Eq,
    And,
    Or,
}

enum BreakpointNode {
    Uninit(String),
    Reg {
        name: String,
        value: u16
    },
    UnaryExpr {
        op: Operator,
        child: Box<BreakpointNode>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<BreakpointNode>,
        rhs: Box<BreakpointNode>,
    },
}

struct RootNode {
    expr: String,
    root: Option<BreakpointNode>,
}

impl From<&str> for RootNode {
    fn from(input: &str) -> Self {
        Self {
            expr: input.into_string(),
            root: None,
        }
    }
}
