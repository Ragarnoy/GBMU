use super::{math, MicrocodeController, MicrocodeFlow, State, CONTINUE};
use crate::interfaces::{ReadFlagReg, WriteFlagReg};

pub fn sub(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let (value, flag) = math::sub_components(ctl.pop(), ctl.pop(), false);
    state.regs.set_subtraction(true);
    state.regs.set_zero(flag.zero);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    ctl.push(value);
    CONTINUE
}

pub fn add(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let (value, flag) = math::add_components(ctl.pop(), ctl.pop(), false);
    state.regs.set_subtraction(false);
    state.regs.set_zero(flag.zero);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    ctl.push(value);
    CONTINUE
}

pub fn add_16(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let b = ctl.pop_u16();
    let a = ctl.pop_u16();
    let (res, overflow) = a.overflowing_add(b);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry((a & 0xfff) + (b & 0xfff) > 0xfff);
    state.regs.set_carry(overflow);
    ctl.push_u16(res);
    CONTINUE
}

/// Daa perform an operation on a byte to format it in Binary Coded Decimal number (BCD)
/// ### Examples
/// where `A = 0x45` and `B = 0x38`
/// `A + B` = `0x7D`
/// Note: `45 + 38 = 83` in base 10
///
/// Daa will add `0x6` to `0x7D`
/// `0x7D + 0x6 = 0x83`
/// Note: the hex representation of `Daa(A + B) = 0x83` is the result of their hex representation added
/// together in a base10 context
pub fn daa(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ctl.pop();
    let was_a_subtraction = state.regs.subtraction();

    let (value, carry) = if was_a_subtraction {
        daa_subtraction(value, state.regs.carry(), state.regs.half_carry())
    } else {
        daa_addition(value, state.regs.carry(), state.regs.half_carry())
    };

    ctl.push(value);
    state.regs.set_carry(carry);
    state.regs.set_half_carry(false);
    state.regs.set_zero(value == 0);

    CONTINUE
}

fn daa_addition(value: u8, carry: bool, half_carry: bool) -> (u8, bool) {
    let (upper, lower) = slice_byte(value);
    let offset = if carry {
        match (upper, lower, half_carry) {
            (0..=2, 0..=9, false) => 0x60,
            (0..=2, 0xa..=0xf, false) => 0x66,
            (0..=3, 0..=3, true) => 0x66,
            _ => 0,
        }
    } else {
        match (upper, lower, half_carry) {
            (0..=9, 0..=9, false) => 0,
            (0..=8, 0xa..=0xf, false) => 6,
            (0..=9, 0..=3, true) => 6,
            (0xa..=0xf, 0..=9, false) => 0x60,
            (9..=0xf, 0xa..=0xf, false) => 0x66,
            (0xa..=0xf, 0..=3, true) => 0x66,
            _ => 0,
        }
    };
    value.overflowing_add(offset)
}

fn daa_subtraction(value: u8, carry: bool, half_carry: bool) -> (u8, bool) {
    let (upper, lower) = slice_byte(value);
    let offset = if carry {
        match (upper, lower, half_carry) {
            (7..=0xf, 0..=9, false) => 0xa0,
            (6..=0xf, 6..=0xf, true) => 0x9a,
            _ => 0,
        }
    } else {
        match (upper, lower, half_carry) {
            (0..=9, 0..=9, false) => 0,
            (0..=8, 6..=0xf, true) => 0xfa,
            _ => 0,
        }
    };
    let (value, _) = value.overflowing_add(offset);
    (value, carry)
}

/// return the upper/lower bound of a byte
pub fn slice_byte(value: u8) -> (u8, u8) {
    let upper = value >> 4;
    let lower = value & 0xf;
    (upper, lower)
}

#[test]
fn test_daa_addition() {
    assert_eq!(daa_addition(0x7D, false, false), (0x83, false));
}

#[test]
fn test_daa_subtraction() {
    assert_eq!(daa_subtraction(0x4b, false, true), (0x45, false));
}

#[test]
fn test_slice_byte() {
    assert_eq!(slice_byte(0x83), (8, 3));
    assert_eq!(slice_byte(0x6), (0, 6));
    assert_eq!(slice_byte(0x60), (6, 0));
    assert_eq!(slice_byte(0), (0, 0));
}

pub fn adc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let left = ctl.pop();
    let right = ctl.pop();
    let (value, flag) = math::add_components(left, right, state.regs.carry());

    state.regs.set_subtraction(false);
    state.regs.set_zero(flag.zero);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    ctl.push(value);
    CONTINUE
}

pub fn sbc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let left = ctl.pop();
    let right = ctl.pop();
    let (value, flag) = math::sub_components(left, right, state.regs.carry());

    state.regs.set_subtraction(true);
    state.regs.set_zero(flag.zero);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    ctl.push(value);
    CONTINUE
}

pub fn add_sp_i8(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addend = ctl.pop() as i8;
    let sp = ctl.pop_u16();
    let (value, flag) = match addend {
        v if v >= 0 => math::add_components_u16(sp, v as u16),
        v => math::sub_components_u16(sp, v as u16),
    };
    state.regs.set_subtraction(false);
    state.regs.set_zero(false);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    ctl.push_u16(value);

    CONTINUE
}
