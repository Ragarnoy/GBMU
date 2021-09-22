use super::{ControlFlow, MicrocodeController, State};
use crate::interfaces::ReadFlagReg;

pub fn carry(_clt: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    if state.regs.carry() {
        ControlFlow::Chain
    } else {
        ControlFlow::Break
    }
}

pub fn not_carry(_clt: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    if !state.regs.carry() {
        ControlFlow::Chain
    } else {
        ControlFlow::Break
    }
}

pub fn zero(_clt: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    if state.regs.zero() {
        ControlFlow::Chain
    } else {
        ControlFlow::Break
    }
}

pub fn not_zero(_clt: &mut MicrocodeController, state: &mut State) -> ControlFlow {
    if !state.regs.zero() {
        ControlFlow::Chain
    } else {
        ControlFlow::Break
    }
}
