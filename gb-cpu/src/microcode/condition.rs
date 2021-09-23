use super::{CycleDigest, MicrocodeController, MicrocodeFlow, State};
use crate::interfaces::ReadFlagReg;

pub fn carry(_clt: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if state.regs.carry() {
        MicrocodeFlow::Continue(CycleDigest::Again)
    } else {
        MicrocodeFlow::Break(CycleDigest::Again)
    }
}

pub fn not_carry(_clt: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if !state.regs.carry() {
        MicrocodeFlow::Continue(CycleDigest::Again)
    } else {
        MicrocodeFlow::Break(CycleDigest::Again)
    }
}

pub fn zero(_clt: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if state.regs.zero() {
        MicrocodeFlow::Continue(CycleDigest::Again)
    } else {
        MicrocodeFlow::Break(CycleDigest::Again)
    }
}

pub fn not_zero(_clt: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    if !state.regs.zero() {
        MicrocodeFlow::Continue(CycleDigest::Again)
    } else {
        MicrocodeFlow::Break(CycleDigest::Again)
    }
}
