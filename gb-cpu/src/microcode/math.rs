use super::flag::Flag;

/// Subtract `b` to `a` (`a - b`).
/// Return a Flag set of triggered flag.
pub fn sub_components(a: u8, b: u8) -> (u8, Flag) {
    let (res, overflowing) = a.overflowing_sub(b);
    (
        res,
        Flag {
            half_carry: (a & 0xf) < (b & 0xf),
            carry: overflowing,
            negative: true,
            zero: res == 0,
        },
    )
}

#[test]
fn test_sub_components() {
    assert_eq!(
        sub_components(4, 2),
        (
            2,
            Flag {
                half_carry: false,
                carry: false,
                negative: true,
                zero: false
            }
        )
    );
    assert_eq!(
        sub_components(2, 4),
        (
            0xff - 2 + 1,
            Flag {
                half_carry: true,
                carry: true,
                negative: true,
                zero: false,
            }
        )
    );
    assert_eq!(
        sub_components(2, 2),
        (
            0,
            Flag {
                half_carry: false,
                carry: false,
                negative: true,
                zero: true
            }
        )
    );
    assert_eq!(
        sub_components(5, 34),
        (
            0xff - 29 + 1,
            Flag {
                half_carry: false,
                carry: true,
                negative: true,
                zero: false,
            }
        )
    )
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
