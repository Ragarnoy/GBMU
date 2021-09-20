use super::{Continuum, MicrocodeController, State};
use crate::interfaces::ReadFlagReg;
use gb_bus::Bus;

pub fn carry<B: Bus<u8>>(_clt: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    if state.regs.carry() {
        Continuum::Chain
    } else {
        Continuum::Break
    }
}

pub fn not_carry<B: Bus<u8>>(_clt: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    if !state.regs.carry() {
        Continuum::Chain
    } else {
        Continuum::Break
    }
}

pub fn zero<B: Bus<u8>>(_clt: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    if state.regs.zero() {
        Continuum::Chain
    } else {
        Continuum::Break
    }
}

pub fn not_zero<B: Bus<u8>>(_clt: &mut MicrocodeController<B>, state: &mut State<B>) -> Continuum {
    if !state.regs.zero() {
        Continuum::Chain
    } else {
        Continuum::Break
    }
}
