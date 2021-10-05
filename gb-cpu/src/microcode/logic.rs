use super::{ident, math, MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION};
use crate::interfaces::{Read8BitsReg, Write8BitsReg, WriteFlagReg};

pub fn cp(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ident::get_u8_from_ident(*ctl.get_src(), state, ctl);
    let (_, flag) = math::sub_components(state.regs.a(), value);
    state.regs.set_subtraction(true);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    state.regs.set_zero(flag.zero);
    OK_PLAY_NEXT_ACTION
}

pub fn xor(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = ident::get_u8_from_ident(*ctl.get_src(), state, ctl);
    let value = state.regs.a() ^ value;
    state.regs.set_raw(0);
    state.regs.set_zero(value == 0);
    state.regs.set_a(value);
    OK_PLAY_NEXT_ACTION
}
