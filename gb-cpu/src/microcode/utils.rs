use super::{MicrocodeController, MicrocodeFlow, State, CONTINUE};

pub fn sleep(_ctl: &mut MicrocodeController, _state: &mut State) -> MicrocodeFlow {
    CONTINUE
}
