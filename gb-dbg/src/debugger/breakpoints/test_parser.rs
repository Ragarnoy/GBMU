use super::parser::{
    address, bin_op, comb_op, expr_complete, operation, raw_register, raw_value, unary_expr,
    unary_expr_id,
};

use crate::debugger::breakpoints::breakpoint::{Operator, UnaryOperator};

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
fn test_raw_register() {
    assert_eq!(raw_register("AF"), Ok(("", CpuRegs::AF)));
    assert_eq!(raw_register("BC"), Ok(("", CpuRegs::BC)));
    assert_eq!(raw_register("DE"), Ok(("", CpuRegs::DE)));
    assert_eq!(raw_register("HL"), Ok(("", CpuRegs::HL)));
    assert_eq!(raw_register("PC"), Ok(("", CpuRegs::PC)));
    assert_eq!(raw_register("SP"), Ok(("", CpuRegs::SP)));
}

#[test]
fn test_comb_op() {
    assert_eq!(comb_op("&&"), Ok(("", Operator::LogicAnd)));
    assert_eq!(comb_op("||"), Ok(("", Operator::LogicOr)));
    assert_eq!(comb_op("^^"), Ok(("", Operator::LogicXor)));
}

#[test]
fn test_raw_value() {
    assert_eq!(raw_value("1"), Ok(("", 0x1_u16)));
    assert_eq!(raw_value("1f"), Ok(("", 0x1f_u16)));
    assert_eq!(raw_value("b1f"), Ok(("", 0xb1f_u16)));
    assert_eq!(raw_value("ab1f"), Ok(("", 0xab1f_u16)));
}

#[test]
fn test_address() {
    assert_eq!(address("*1"), Ok(("", Node::Address(1))));
    assert_eq!(address("*dead"), Ok(("", Node::Address(0xdead))));
}

#[test]
fn test_unary_expr_id() {
    assert_eq!(unary_expr_id("U"), Ok(("", UnaryOperator::Upper)));
    assert_eq!(unary_expr_id("L"), Ok(("", UnaryOperator::Lower)));
}

#[test]
fn test_unary_expr() {
    assert_eq!(
        unary_expr("U(AF)"),
        Ok((
            "",
            Node::UnaryExpr {
                op: UnaryOperator::Upper,
                child: crate::boxed!(Node::Register(CpuRegs::AF))
            }
        ))
    );
}

#[cfg(test)]
fn utils_test_expr<'a, P>(parser: P, input: &'a str, expected: &str)
where
    P: nom::Parser<&'a str, Node, nom::error::Error<&'a str>>,
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
    use super::{operation, utils_test_expr};

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
    use super::{expr_complete, utils_test_expr};
    #[test]
    fn no_space() {
        utils_test_expr(expr_complete, "AF==42", "AF == 0x42");
        utils_test_expr(
            expr_complete,
            "AF==21||PC==dead",
            "AF == 0x21 || PC == 0xDEAD",
        );
    }
    #[test]
    fn space() {
        utils_test_expr(expr_complete, "AF ==42", "AF == 0x42");
        utils_test_expr(
            expr_complete,
            "AF== 21 ||PC== dead",
            "AF == 0x21 || PC == 0xDEAD",
        );
    }
}
