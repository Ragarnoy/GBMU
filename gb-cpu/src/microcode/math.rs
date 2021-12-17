use super::flag::Flag;

#[cfg(test)]
macro_rules! flag {
    () => {
        Flag::default()
    };
    ($($flag:ident),*) => {
        Flag {
            $($flag: true,)*
            ..Default::default()
        }
    }
}

/// Subtract `b` to `a` (`a - b`).
/// Return a Flag set of triggered flag.
pub fn sub_components(a: u8, b: u8, borrow: bool) -> (u8, Flag) {
    let borrow = u8::from(borrow);
    let (res, borrowing_1) = a.overflowing_sub(b);
    let (res, borrowing_2) = res.overflowing_sub(borrow);
    (
        res,
        Flag {
            half_carry: (a & 0xf) < (b & 0xf) + borrow,
            carry: borrowing_1 || borrowing_2,
            negative: true,
            zero: res == 0,
        },
    )
}

#[test]
fn test_sub_components() {
    macro_rules! sub {
        (($a:expr, $b:expr, $carry:expr, $res:expr) $($flag:ident)|*) => {
            assert_eq!(sub_components($a, $b, $carry), ($res, flag!(negative $(,$flag)*)))
        }
    }
    sub!((4, 2, false, 2));
    sub!((2, 4, false, u8::MAX - 2 + 1) half_carry | carry);
    sub!((2,2,false,0) zero);
    sub!((5, 34, false, u8::MAX - 29 + 1) carry);
    sub!((5, 2, false, 3));
    sub!((0, 1, false, u8::MAX) carry | half_carry);
    sub!((5, 2, true, 2));
    sub!((0, 1, true, u8::MAX - 1) carry | half_carry);
}


/// Add `b` to `a` (`a + b`)
/// Return a Flag set of triggered flag.
pub fn add_components(a: u8, b: u8, carry: bool) -> (u8, Flag) {
    let carry = u8::from(carry);
    let (res, overflowing_1) = a.overflowing_add(b);
    let (res, overflowing_2) = res.overflowing_add(carry);
    (
        res,
        Flag {
            half_carry: (a & 0xf) + (b & 0xf) + carry > 0xf,
            carry: overflowing_1 || overflowing_2,
            negative: false,
            zero: res == 0,
        },
    )
}

#[test]
fn test_add_components() {
    macro_rules! add {
        (($a:expr, $b:expr, $carry:expr, $res:expr) $($flag:ident)|*) => {
            assert_eq!(add_components($a, $b, $carry), ($res, flag!($($flag),*)))
        }
    }
    add!((4, 4, false, 8));
    add!((0, 0xf, true, 0x10) half_carry);
}

/// Add `b` to `a` (`a + b`)
/// Return a Flag set of triggered flag.
pub fn add_components_u16(a: u16, b: u16) -> (u16, Flag) {
    let (res, overflowing) = a.overflowing_add(b);
    (
        res,
        Flag {
            half_carry: (a & 0xfff) + (b & 0xfff) > 0xfff,
            carry: overflowing,
            negative: false,
            zero: res == 0,
        },
    )
}

#[test]
fn test_add_components_u16() {
    macro_rules! add {
        (($a:expr, $b:expr, $res:expr) $($flag:ident)|*) => {
            assert_eq!(add_components_u16($a, $b), ($res, flag!($($flag),*)))
        }
    }
    add!((4, 4, 8));
    add!((u16::MAX, 1, 0) half_carry | carry | zero);
    add!((0xff, 1, 0x100) half_carry);
}
