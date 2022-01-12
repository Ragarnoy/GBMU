use nom::{combinator::map, IResult};

use crate::Ast;

/// Wrap [crate::register::Register] in [Ast::Register]
pub fn wrap_register(input: &str) -> IResult<&str, Ast> {
    let res = map(crate::register::register, Ast::Register)(input);
    eprintln!("[wrap_register] res: {:?}", res);
    res
}

/// Wrap [crate::native::value] in [Ast::Raw]
pub fn wrap_value(input: &str) -> IResult<&str, Ast> {
    map(crate::native::value, Ast::Raw)(input)
}

/// Wrap [crate::unary::unary_expr] to [Ast::UnaryExpr]
pub fn wrap_unary(input: &str) -> IResult<&str, Ast> {
    map(crate::unary::unary_expr, Ast::UnaryExpr)(input)
}

/// Wrap [crate::operation::bin_expr] to [Ast::BinaryExpr]
pub fn wrap_bin_expr(input: &str) -> IResult<&str, Ast> {
    map(crate::operation::bin_expr, Ast::BinaryExpr)(input)
}

/// Wrap [crate::operation::comb_expr] to [Ast::BinaryExpr]
pub fn wrap_comb_expr(input: &str) -> IResult<&str, Ast> {
    map(crate::operation::comb_expr, Ast::BinaryExpr)(input)
}
