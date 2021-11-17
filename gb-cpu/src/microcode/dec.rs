use super::{
    flag::Flag, math::sub_components, MicrocodeController, MicrocodeFlow, State, OK_CONSUME_CYCLE,
    OK_PLAY_NEXT_ACTION,
};
use crate::interfaces::WriteFlagReg;

pub fn dec16(ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    let (value, _) = ctl.pop_u16().overflowing_sub(1);
    ctl.push_u16(value);
    OK_CONSUME_CYCLE
}

pub fn dec8(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let (value, flag) = sub_components(ctl.pop(), 1);
    update_dec_flag(state.regs, flag);
    ctl.push(value);
    OK_PLAY_NEXT_ACTION
}

fn update_dec_flag(state: &mut impl WriteFlagReg, flag: Flag) {
    state.set_half_carry(flag.half_carry);
    state.set_zero(flag.zero);
    state.set_subtraction(true);
}

/// decrease sp by one
pub fn sp(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.sp -= 1;
    OK_PLAY_NEXT_ACTION
}

/// decrease hl by one
pub fn hl(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.hl -= 1;
    OK_PLAY_NEXT_ACTION
}
