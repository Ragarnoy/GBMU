use super::{
    flag::Flag, math::add_components, MicrocodeController, MicrocodeFlow, State, OK_CONSUME_CYCLE,
    OK_PLAY_NEXT_ACTION,
};
use crate::interfaces::WriteFlagReg;

pub fn inc16(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    let (value, _) = ctl.pop_u16().overflowing_add(1);
    ctl.push_u16(value);
    OK_CONSUME_CYCLE
}

pub fn inc8(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let (value, flag) = add_components(ctl.pop(), 1);
    update_inc_flag(state.regs, flag);
    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}

fn update_inc_flag(state: &mut impl WriteFlagReg, flag: Flag) {
    state.set_subtraction(false);
    state.set_half_carry(flag.half_carry);
    state.set_zero(flag.zero);
}

/// increase sp by one
pub fn sp(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.sp += 1;
    OK_PLAY_NEXT_ACTION
}

/// increase `HL` without consuming the cycle
pub fn hl(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.hl += 1;
    OK_PLAY_NEXT_ACTION
}
