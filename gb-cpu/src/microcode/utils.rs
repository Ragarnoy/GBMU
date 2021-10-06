use super::{MicrocodeController, MicrocodeFlow, State, OK_CONSUME_CYCLE};

pub fn sleep(_ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    OK_CONSUME_CYCLE
}
