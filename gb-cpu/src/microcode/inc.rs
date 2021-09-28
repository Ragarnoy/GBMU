use super::{
    ident::{self, Ident},
    math::add_components,
    MicrocodeController, MicrocodeFlow, State, OK_CONSUME_CYCLE, OK_PLAY_NEXT_ACTION,
};
use crate::interfaces::{Read8BitsReg, Write8BitsReg, WriteFlagReg};

pub fn inc_hl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let (val, flag) = add_components(ctl.pop(), 1);
    state.regs.set_subtraction(false);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_zero(flag.zero);
    ctl.push(val);
    OK_PLAY_NEXT_ACTION
}

pub fn inc16(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if let Ident::Reg16(r16) = ctl.get_dest() {
        use ident::Reg16;
        match r16 {
            Reg16::BC => state.regs.bc += 1,
            Reg16::DE => state.regs.de += 1,
            Reg16::HL => state.regs.hl += 1,
            Reg16::SP => state.regs.sp += 1,
        }
        OK_CONSUME_CYCLE
    } else {
        panic!("call inc16 with something other than a reg16");
    }
}

macro_rules! inc_reg8 {
    ($state: expr, $setter: ident, $getter: ident) => {{
        let (val, flag) = add_components($state.regs.$getter(), 1);
        $state.regs.set_zero(flag.zero);
        $state.regs.set_half_carry(flag.half_carry);
        $state.regs.set_subtraction(false);
        $state.regs.$setter(val);
    }};
}

pub fn inc8(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
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
        OK_PLAY_NEXT_ACTION
    } else {
        panic!("call inc8 with something other than a reg8");
    }
}
