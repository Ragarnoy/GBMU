use super::{math, MicrocodeController, MicrocodeFlow, State, CONTINUE};
use crate::interfaces::{ReadFlagReg, WriteFlagReg};

pub fn sub(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let (value, flag) = math::sub_components(ctl.pop(), ctl.pop());
    state.regs.set_subtraction(true);
    state.regs.set_zero(flag.zero);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    ctl.push(value);
    CONTINUE
}

pub fn add(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let (value, flag) = math::add_components(ctl.pop(), ctl.pop());
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
    let mut a = ctl.pop();
    let mut carry = state.regs.carry();
    let half_carry = state.regs.half_carry();
    let was_a_subtraction = state.regs.subtraction();

    if !was_a_subtraction {
        if carry || a > 0x99 {
            a += 0x60;
            carry = true;
        }
        if half_carry || (a & 0x0f) > 0x09 {
            a += 0x6;
        }
    } else {
        if carry {
            a -= 0x60;
        }
        if half_carry {
            a -= 0x6;
        }
    }

    ctl.push(a);
    state.regs.set_carry(carry);
    state.regs.set_half_carry(false);
    state.regs.set_zero(a == 0);

    CONTINUE
}

/// return the upper/lower bound of a byte
pub fn slice_byte(value: u8) -> (u8, u8) {
    let upper = value >> 4;
    let lower = value & 0xf;
    (upper, lower)
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
    let right = ctl.pop() + (state.regs.carry() as u8);
    let (value, flag) = math::add_components(left, right);

    state.regs.set_subtraction(false);
    state.regs.set_zero(flag.zero);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    ctl.push(value);
    CONTINUE
}

pub fn sbc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let left = ctl.pop();
    let right = ctl.pop() + (state.regs.carry() as u8);
    let (value, flag) = math::sub_components(left, right);

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
