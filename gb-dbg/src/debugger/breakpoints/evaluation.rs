use crate::dbg_interfaces::{CpuRegs, DebugOperations};
use gb_breakpoint::{Ast, Operator, UnaryOperator};

const TRUE: u16 = 0xffff;
const FALSE: u16 = 0x0000;

pub fn is_expression_true<DBG: DebugOperations>(node: &Ast, dbg: &DBG) -> bool {
    compute_expression(node, dbg) != FALSE
}

pub fn compute_expression<DBG: DebugOperations>(node: &Ast, dbg: &DBG) -> u16 {
    let current = node;

    match current {
        Ast::Register(r) => u16::from(dbg.cpu_get(CpuRegs::try_from(*r).unwrap())),
        Ast::Address(a) => u16::from(compute_expression(a, dbg)),
        Ast::Raw(v) => *v,
        Ast::UnaryExpr(expr) => eval_unary_op(&expr.op, compute_expression(&expr.child, dbg)),
        Ast::BinaryExpr(expr) => eval_binary_op(
            &expr.op,
            compute_expression(&expr.lhs, dbg),
            compute_expression(&expr.rhs, dbg),
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
            if lhs != FALSE && rhs == FALSE || lhs == FALSE && rhs != FALSE {
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
