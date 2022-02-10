use nom::{combinator::map, IResult};

use crate::Node;

/// Wrap [crate::register::Register] in [Ast::Register]
pub fn wrap_register(input: &str) -> IResult<&str, Node> {
    let res = map(crate::register::register, Node::Register)(input);
    eprintln!("[wrap_register] res: {:?}", res);
    res
}

/// Wrap [crate::native::value] in [Ast::Raw]
pub fn wrap_value(input: &str) -> IResult<&str, Node> {
    map(crate::native::value, Node::Raw)(input)
}

/// Wrap [crate::unary::unary_expr] to [Ast::UnaryExpr]
pub fn wrap_unary(input: &str) -> IResult<&str, Node> {
    map(crate::unary::unary_expr, Node::UnaryExpr)(input)
}

/// Wrap [crate::operation::bin_expr] to [Ast::BinaryExpr]
pub fn wrap_bin_expr(input: &str) -> IResult<&str, Node> {
    map(crate::operation::bin_expr, Node::BinaryExpr)(input)
}

/// Wrap [crate::operation::comb_expr] to [Ast::BinaryExpr]
pub fn wrap_comb_expr(input: &str) -> IResult<&str, Node> {
    map(crate::operation::comb_expr, Node::BinaryExpr)(input)
}
