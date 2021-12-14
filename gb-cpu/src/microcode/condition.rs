use super::{MicrocodeController, MicrocodeFlow, State, BREAK, CONTINUE};
use crate::interfaces::ReadFlagReg;

pub fn carry(_clt: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if state.regs.carry() {
        CONTINUE
    } else {
        BREAK
    }
}

pub fn not_carry(_clt: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if !state.regs.carry() {
        CONTINUE
    } else {
        BREAK
    }
}

pub fn zero(_clt: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if state.regs.zero() {
        CONTINUE
    } else {
        BREAK
    }
}

pub fn not_zero(_clt: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if !state.regs.zero() {
        CONTINUE
    } else {
        BREAK
    }
}
