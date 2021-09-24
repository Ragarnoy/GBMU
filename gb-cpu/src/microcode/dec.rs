use super::{
    flag::Flag,
    ident::{self, Ident},
    MicrocodeController, MicrocodeFlow, State, OK_CONSUME_CYCLE, OK_PLAY_NEXT_ACTION,
};
use crate::interfaces::{Read8BitsReg, Write8BitsReg};

pub fn dec_hl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let (val, flag) = sub_reg_flags(ctl.pop(), 1);
    flag.update_reg_flag(state.regs);
    ctl.push(val);
    OK_PLAY_NEXT_ACTION
}

pub fn dec16(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if let Ident::Reg16(r16) = ctl.get_dest() {
        use ident::Reg16;
        match r16 {
            Reg16::BC => state.regs.bc -= 1,
            Reg16::DE => state.regs.de -= 1,
            Reg16::HL => state.regs.hl -= 1,
            Reg16::SP => state.regs.sp -= 1,
        }
        OK_CONSUME_CYCLE
    } else {
        panic!("call dec16 with something other than a reg16");
    }
}

macro_rules! dec_reg8 {
    ($state: expr, $setter: ident, $getter: ident) => {{
        let (val, flag) = sub_reg_flags($state.regs.$getter(), 1);
        flag.update_reg_flag($state.regs);
        $state.regs.$setter(val);
    }};
}

pub fn dec8(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if let Ident::Reg8(r8) = ctl.get_dest() {
        use ident::Reg8;
        match r8 {
            Reg8::A => dec_reg8!(state, set_a, a),
            Reg8::B => dec_reg8!(state, set_b, b),
            Reg8::C => dec_reg8!(state, set_c, c),
            Reg8::D => dec_reg8!(state, set_d, d),
            Reg8::E => dec_reg8!(state, set_e, e),
            Reg8::H => dec_reg8!(state, set_h, h),
            Reg8::L => dec_reg8!(state, set_l, l),
        }
        OK_PLAY_NEXT_ACTION
    } else {
        panic!("call dec8 with something other than a reg8");
    }
}

/// Add `amount` to `value`.
/// Return a Flag set of triggered flag.
/// PS: the flag `carry` is not used here
fn sub_reg_flags(value: u8, amount: u8) -> (u8, Flag) {
    let (res, _) = value.overflowing_sub(amount);
    (res, Flag::from_values(value, res, true, None))
}

#[test]
fn test_sub_reg_flags() {
    assert_eq!(
        sub_reg_flags(0xff, 1),
        (
            0xfe,
            Flag {
                half_carry: Some(true),
                carry: None,
                negative: Some(true),
                zero: Some(false),
            }
        )
    );
    assert_eq!(
        sub_reg_flags(0x10, 1),
        (
            0xf,
            Flag {
                half_carry: Some(false),
                carry: None,
                negative: Some(true),
                zero: Some(false),
            }
        )
    );
    assert_eq!(
        sub_reg_flags(0x0, 1),
        (
            0xff,
            Flag {
                half_carry: Some(false),
                carry: None,
                negative: Some(true),
                zero: Some(false),
            }
        )
    );
    assert_eq!(
        sub_reg_flags(0, 0),
        (
            0,
            Flag {
                half_carry: Some(false),
                carry: None,
                negative: Some(true),
                zero: Some(true),
            }
        )
    );
}