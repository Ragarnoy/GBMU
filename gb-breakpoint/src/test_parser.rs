use crate::parser::address;
use crate::{boxed, Node};

#[test]
fn test_address() {
    assert_eq!(address("*1"), Ok(("", Node::Address(boxed!(Node::Raw(1))))));
    assert_eq!(
        address("*dead"),
        Ok(("", Node::Address(boxed!(Node::Raw(0xdead)))))
    );
}

#[cfg(test)]
#[macro_export]
macro_rules! utils_test_expr {
    ($parser:expr, $input:expr, $expected:expr) => {{
        let res = nom::combinator::all_consuming($parser)($input);
        assert!(res.is_ok(), "for `{}': res is not ok: {:?}", $input, res);
        let (left, expr) = res.unwrap();
        assert!(left.is_empty(), "data still need to be proceded: {}", left);
        assert_eq!(
            expr.to_string(),
            $expected,
            "failed to correct represent with {:?}",
            expr
        );
    }};
}

#[cfg(test)]
mod unit_expr {
    use crate::parser::expr;
    #[test]
    fn no_space() {
        utils_test_expr!(expr, "AF==42", "AF == 0x42");
        utils_test_expr!(expr, "AF==21||PC==dead", "AF == 0x21 || PC == 0xDEAD");
    }

    #[test]
    fn space() {
        utils_test_expr!(expr, "AF ==42", "AF == 0x42");
        utils_test_expr!(expr, "AF== 21 ||PC== dead", "AF == 0x21 || PC == 0xDEAD");
    }

    #[test]
    fn simple() {
        utils_test_expr!(expr, "HL == b000", "HL == 0xB000");
        utils_test_expr!(expr, "*4088 == e3", "*0x4088 == 0xE3");
    }
}
