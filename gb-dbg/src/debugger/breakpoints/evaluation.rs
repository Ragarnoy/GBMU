use crate::dbg_interfaces::DebugOperations;
use crate::debugger::breakpoints::breakpoint::{Node, Operator, UnaryOperator};

const TRUE: u16 = 0xffff;
const FALSE: u16 = 0x0000;

pub fn compute_expression<DBG: DebugOperations>(node: &Node, dbg: &DBG) -> u16 {
    let current = node;

    match current {
        Node::Register(r) => u16::from(dbg.cpu_get(*r)),
        Node::Address(a) => u16::from(dbg.read(*a)),
        Node::Value(v) => *v,
        Node::UnaryExpr { op, child } => eval_unary_op(op, compute_expression(child, dbg)),
        Node::BinaryExpr { op, lhs, rhs } => eval_binary_op(
            op,
            compute_expression(lhs, dbg),
            compute_expression(rhs, dbg),
        ),
    }
}

pub fn eval_unary_op(op: &UnaryOperator, value: u16) -> u16 {
    match op {
        UnaryOperator::Upper => value.to_le_bytes()[1] as u16,
        UnaryOperator::Lower => value.to_le_bytes()[0] as u16,
    }
}

pub fn eval_binary_op(op: &Operator, lhs: u16, rhs: u16) -> u16 {
    match op {
        Operator::Eq => {
            if lhs == rhs {
                TRUE
            } else {
                FALSE
            }
        }
        Operator::LogicAnd => {
            if lhs != FALSE && rhs != FALSE {
                TRUE
            } else {
                FALSE
            }
        }
        Operator::LogicXor => {
            if lhs == TRUE && rhs == FALSE {
                TRUE
            } else if lhs == FALSE && rhs == TRUE {
                TRUE
            } else {
                FALSE
            }
        }
        Operator::LogicOr => {
            if lhs != FALSE || rhs != FALSE {
                TRUE
            } else {
                FALSE
            }
        }
        Operator::BinaryAnd => lhs & rhs,
        Operator::BinaryXor => lhs ^ rhs,
        Operator::BinaryOr => lhs | rhs,
        Operator::NotEq => {
            if lhs != rhs {
                TRUE
            } else {
                FALSE
            }
        }
        Operator::Sup => {
            if lhs > rhs {
                TRUE
            } else {
                FALSE
            }
        }
        Operator::Inf => {
            if lhs < rhs {
                TRUE
            } else {
                FALSE
            }
        }
        Operator::SupEq => {
            if lhs >= rhs {
                TRUE
            } else {
                FALSE
            }
        }
        Operator::InfEq => {
            if lhs <= rhs {
                TRUE
            } else {
                FALSE
            }
        }
    }
}
