use super::{ControlFlow, MicrocodeController, State};
use crate::interfaces::ReadFlagReg;
use gb_bus::Bus;

pub fn carry<B: Bus<u8>>(_clt: &mut MicrocodeController<B>, state: &mut State<B>) -> ControlFlow {
    if state.regs.carry() {
        ControlFlow::Chain
    } else {
        ControlFlow::Break
    }
}

pub fn not_carry<B: Bus<u8>>(
    _clt: &mut MicrocodeController<B>,
    state: &mut State<B>,
) -> ControlFlow {
    if !state.regs.carry() {
        ControlFlow::Chain
    } else {
        ControlFlow::Break
    }
}

pub fn zero<B: Bus<u8>>(_clt: &mut MicrocodeController<B>, state: &mut State<B>) -> ControlFlow {
    if state.regs.zero() {
        ControlFlow::Chain
    } else {
        ControlFlow::Break
    }
}

pub fn not_zero<B: Bus<u8>>(
    _clt: &mut MicrocodeController<B>,
    state: &mut State<B>,
) -> ControlFlow {
    if !state.regs.zero() {
        ControlFlow::Chain
    } else {
        ControlFlow::Break
    }
}
