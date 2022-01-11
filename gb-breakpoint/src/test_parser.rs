use crate::breakpoint::Operator;
use crate::parser::{address, bin_op, comb_op};
use crate::Ast;

#[test]
fn test_bin_op() {
    assert_eq!(bin_op("=="), Ok(("", Operator::Eq)));
    assert_eq!(bin_op("!="), Ok(("", Operator::NotEq)));
    assert_eq!(bin_op(">="), Ok(("", Operator::SupEq)));
    assert_eq!(bin_op("<="), Ok(("", Operator::InfEq)));
    assert_eq!(bin_op("<"), Ok(("", Operator::Inf)));
    assert_eq!(bin_op(">"), Ok(("", Operator::Sup)));
    assert_eq!(bin_op("|"), Ok(("", Operator::BinaryOr)));
    assert_eq!(bin_op("&"), Ok(("", Operator::BinaryAnd)));
    assert_eq!(bin_op("^"), Ok(("", Operator::BinaryXor)));
}

#[test]
fn test_comb_op() {
    assert_eq!(comb_op("&&"), Ok(("", Operator::LogicAnd)));
    assert_eq!(comb_op("||"), Ok(("", Operator::LogicOr)));
    assert_eq!(comb_op("^^"), Ok(("", Operator::LogicXor)));
}

#[test]
fn test_address() {
    assert_eq!(address("*1"), Ok(("", Ast::Address(1))));
    assert_eq!(address("*dead"), Ok(("", Ast::Address(0xdead))));
}

#[cfg(test)]
fn utils_test_expr<'a, P>(parser: P, input: &'a str, expected: &str)
where
    P: nom::Parser<&'a str, Ast, nom::error::Error<&'a str>>,
{
    use nom::combinator::all_consuming;

    let res = all_consuming(parser)(input);
    assert!(res.is_ok(), "for `{}': res is not ok: {:?}", input, res);
    let (left, expr) = res.unwrap();
    assert!(left.is_empty(), "data still need to be proceded: {}", left);
    assert_eq!(expr.to_string(), expected);
}

#[cfg(test)]
mod unit_operation {
    use super::utils_test_expr;
    use crate::parser::operation;

    #[test]
    fn no_space() {
        utils_test_expr(operation, "AF==42", "AF == 0x42");
        utils_test_expr(operation, "SP<=fffe", "SP <= 0xFFFE");
        utils_test_expr(operation, "HL!=*ff0f", "HL != *0xFF0F");
        utils_test_expr(operation, "HL<DE", "HL < DE");
    }

    #[test]
    fn space() {
        utils_test_expr(operation, "AF ==42", "AF == 0x42");
        utils_test_expr(operation, "SP<= fffe", "SP <= 0xFFFE");
        utils_test_expr(operation, "HL != *ff0f", "HL != *0xFF0F");
        utils_test_expr(operation, "HL < DE", "HL < DE");
    }
}

#[cfg(test)]
mod unit_expr {
    use super::utils_test_expr;
    use crate::parser::expr;
    #[test]
    fn no_space() {
        utils_test_expr(expr, "AF==42", "AF == 0x42");
        utils_test_expr(expr, "AF==21||PC==dead", "AF == 0x21 || PC == 0xDEAD");
    }

    #[test]
    fn space() {
        utils_test_expr(expr, "AF ==42", "AF == 0x42");
        utils_test_expr(expr, "AF== 21 ||PC== dead", "AF == 0x21 || PC == 0xDEAD");
    }

    #[test]
    fn simple() {
        utils_test_expr(expr, "HL == b000", "HL == 0xB000");
        utils_test_expr(expr, "*4088 == e3", "*0x4088 == 0xE3");
    }
}
