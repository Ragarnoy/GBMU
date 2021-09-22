use super::{
    ident::{self, Ident},
    ControlFlow, MicrocodeController, State,
};
use crate::interfaces::{Read8BitsReg, Write8BitsReg, WriteFlagReg};

pub fn inc_hl(_ctl: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    let (val, flag) = add_reg_flags(state.read_hl(), 1);
    flag.update_reg_flag(state.regs);
    state.write_hl(val);
    ControlFlow::Ok
}

pub fn inc16(ctl: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    if let Ident::Reg16(r16) = ctl.get_dest() {
        use ident::Reg16;
        match r16 {
            Reg16::BC => state.regs.bc += 1,
            Reg16::DE => state.regs.de += 1,
            Reg16::HL => state.regs.hl += 1,
            Reg16::SP => state.regs.sp += 1,
        }
        ControlFlow::Ok
    } else {
        panic!("call inc16 with something other than a reg16");
    }
}

macro_rules! inc_reg8 {
    ($state: expr, $setter: ident, $getter: ident) => {{
        let (val, flag) = add_reg_flags($state.regs.$getter(), 1);
        flag.update_reg_flag($state.regs);
        $state.regs.$setter(val);
    }};
}

pub fn inc8(ctl: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    if let Ident::Reg8(r8) = ctl.get_dest() {
        use ident::Reg8;
        match r8 {
            Reg8::A => inc_reg8!(state, set_a, a),
            Reg8::B => inc_reg8!(state, set_b, b),
            Reg8::C => inc_reg8!(state, set_c, c),
            Reg8::D => inc_reg8!(state, set_d, d),
            Reg8::E => inc_reg8!(state, set_e, e),
            Reg8::H => inc_reg8!(state, set_h, h),
            Reg8::L => inc_reg8!(state, set_l, l),
        }
        ControlFlow::Ok
    } else {
        panic!("call inc8 with something other than a reg8");
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Flag {
    half_carry: Option<bool>,
    carry: Option<bool>,
    negative: Option<bool>,
    zero: Option<bool>,
}

impl Flag {
    pub fn new(
        half_carry: Option<bool>,
        carry: Option<bool>,
        negative: Option<bool>,
        zero: Option<bool>,
    ) -> Self {
        Self {
            half_carry,
            carry,
            negative,
            zero,
        }
    }

    fn update_reg_flag<F: WriteFlagReg>(&self, flag: &mut F) {
        if let Some(hcarry) = self.half_carry {
            flag.set_half_carry(hcarry)
        }
        if let Some(carry) = self.carry {
            flag.set_carry(carry)
        }
        if let Some(negative) = self.negative {
            flag.set_subtraction(negative)
        }
        if let Some(zero) = self.zero {
            flag.set_zero(zero)
        }
    }
}

/// Add `amount` to `value`.
/// Return a Flag set of triggered flag.
/// PS: the flag `carry` is not used here
fn add_reg_flags(value: u8, amount: u8) -> (u8, Flag) {
    let (res, _) = value.overflowing_add(amount);
    (
        res,
        Flag::new(
            Some((value & 0xF) > (res & 0xF)),
            None,
            Some(false),
            Some(res == 0),
        ),
    )
}

#[test]
fn test_add_reg_flags() {
    assert_eq!(
        add_reg_flags(0xff, 1),
        (0, Flag::new(Some(true), None, Some(false), Some(true)))
    );
    assert_eq!(
        add_reg_flags(0xf, 1),
        (0x10, Flag::new(Some(true), None, Some(false), Some(false)))
    );
    assert_eq!(
        add_reg_flags(0x0, 1),
        (1, Flag::new(Some(false), None, Some(false), Some(false)))
    );
    assert_eq!(
        add_reg_flags(0, 0),
        (0, Flag::new(Some(false), None, Some(false), Some(true)))
    );
}
