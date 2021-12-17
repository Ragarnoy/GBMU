use super::flag::Flag;

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
    macro_rules! sub {
        (($a:literal, $b:literal, $carry:literal, $res:expr) $($flag:ident)|*) => {
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

pub fn sub_components_u16(a: u16, b: u16) -> (u16, Flag) {
    let (res, overflowing) = a.overflowing_sub(b);
    (
        res,
        Flag {
            half_carry: (a & 0xff) < (b & 0xff),
            carry: overflowing,
            negative: true,
            zero: res == 0,
        },
    )
}

#[test]

fn test_sub_components_u16() {
    assert_eq!(
        sub_components_u16(2, 2),
        (
            0,
            Flag {
                half_carry: false,
                carry: false,
                negative: true,
                zero: true,
            }
        )
    );
    assert_eq!(
        sub_components_u16(2, 4),
        (
            0xFFFF - 2 + 1,
            Flag {
                half_carry: true,
                carry: true,
                negative: true,
                zero: false,
            }
        )
    );
    assert_eq!(
        sub_components_u16(0x00FF, 0xFF00),
        (
            0x01FF,
            Flag {
                half_carry: false,
                carry: true,
                negative: true,
                zero: false,
            }
        )
    );
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
    assert_eq!(
        add_components(4, 4, false),
        (
            8,
            Flag {
                half_carry: false,
                carry: false,
                negative: false,
                zero: false
            }
        )
    );
    assert_eq!(
        add_components(0, 0xf, true),
        (
            0x10,
            Flag {
                half_carry: true,
                carry: false,
                negative: false,
                zero: false,
            }
        )
    );
}

/// Add `b` to `a` (`a + b`)
/// Return a Flag set of triggered flag.
pub fn add_components_u16(a: u16, b: u16) -> (u16, Flag) {
    let (res, overflowing) = a.overflowing_add(b);
    (
        res,
        Flag {
            half_carry: (a & 0xff) + (b & 0xff) > 0xff,
            carry: overflowing,
            negative: false,
            zero: res == 0,
        },
    )
}

#[test]
fn test_add_components_u16() {
    assert_eq!(
        add_components_u16(4, 4),
        (
            8,
            Flag {
                half_carry: false,
                carry: false,
                negative: false,
                zero: false
            }
        )
    );
    assert_eq!(
        add_components_u16(0xFFFF, 1),
        (
            0,
            Flag {
                half_carry: true,
                carry: true,
                negative: false,
                zero: true
            }
        )
    );
    assert_eq!(
        add_components_u16(0x00FF, 1),
        (
            256,
            Flag {
                half_carry: true,
                carry: false,
                negative: false,
                zero: false
            }
        )
    );
}
