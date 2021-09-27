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
}

/// Add `b` to `a` (`a + b`)
/// Return a Flat set of triggered flag.
pub fn add_componenets(a: u8, b: u8) -> (u8, Flag) {
    let (res, overflowing) = a.overflowing_add(b);
    (
        res,
        Flag {
            half_carry: (a & 0xf) + (b & 0xf) > 0xf,
            carry: overflowing,
            negative: false,
            zero: res == 0,
        },
    )
}
