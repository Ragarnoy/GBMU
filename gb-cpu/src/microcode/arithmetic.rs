use super::{
    ident::get_u8_from_ident, math, MicrocodeController, MicrocodeFlow, State, OK_PLAY_NEXT_ACTION,
};
use crate::interfaces::{Read8BitsReg, Write8BitsReg, WriteFlagReg};

pub fn sub(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let value = get_u8_from_ident(*ctl.get_src(), state, ctl);
    let (value, flag) = math::sub_components(state.regs.a(), value);
    state.regs.set_subtraction(true);
    state.regs.set_zero(flag.zero);
    state.regs.set_half_carry(flag.half_carry);
    state.regs.set_carry(flag.carry);
    state.regs.set_a(value);
    OK_PLAY_NEXT_ACTION
}
