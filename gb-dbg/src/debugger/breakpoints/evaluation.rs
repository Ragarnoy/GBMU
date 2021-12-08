use crate::dbg_interfaces::DebugOperations;
use crate::debugger::breakpoints::breakpoint::{Node, Operator, UnaryOperator};

pub fn compute_expression<DBG: DebugOperations>(node: &Node, dbg: &DBG) {
    let current = node;

    match current {
        Node::Register(r) => {}
        Node::Address(a) => {}
        Node::Value(v) => {}
        Node::UnaryExpr { op, child } => {}
        Node::BinaryExpr { op, lhs, rhs } => {}
    }
}

pub fn eval_unary_op(op: &UnaryOperator, value: u16) -> u16 {
    match op {
        UnaryOperator::Upper => { value.to_le_bytes()[1] as u16 }
        UnaryOperator::Lower => { value.to_le_bytes()[0] as u16 }
    }
}

pub fn eval_binary_op(op: &Operator, lhs: u16, rhs: u16) -> u16 {
    match op {
        Operator::Eq => {}
        Operator::LogicAnd => {}
        Operator::LogicXor => {}
        Operator::LogicOr => {}
        Operator::BinaryAnd => {}
        Operator::BinaryXor => {}
        Operator::BinaryOr => {}
        Operator::NotEq => {}
        Operator::Sup => {}
        Operator::Inf => {}
        Operator::SupEq => {}
        Operator::InfEq => {}
    }
}