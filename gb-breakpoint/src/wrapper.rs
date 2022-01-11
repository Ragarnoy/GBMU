use nom::{combinator::map, IResult};

use crate::Ast;

/// Wrap [crate::register::Register] in [Ast::Register]
pub fn wrap_register(input: &str) -> IResult<&str, Ast> {
    map(crate::register::register, Ast::Register)(input)
}

/// Wrap [crate::native::value] in [Ast::Value]
pub fn wrap_value(input: &str) -> IResult<&str, Ast> {
    map(crate::native::value, Ast::Raw)(input)
}

pub fn wrap_unary(input: &str) -> IResult<&str, Ast> {
    map(crate::unary::unary_expr, Ast::UnaryExpr)(input)
}
