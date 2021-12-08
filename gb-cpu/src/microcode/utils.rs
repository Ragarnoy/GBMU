use super::{MicrocodeController, MicrocodeFlow, State, OK_CONSUME_CYCLE, OK_PLAY_NEXT_ACTION};

pub fn sleep(_ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    OK_CONSUME_CYCLE
}

pub fn inc_pc(_ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    state.regs.pc += 1;
    OK_PLAY_NEXT_ACTION
}
