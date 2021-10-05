use super::{MicrocodeController, MicrocodeFlow, State, OK_CONSUME_CYCLE, OK_PLAY_NEXT_ACTION};
use crate::interfaces::Read8BitsReg;

/// Read a byte and push it to the cache stack
pub fn byte(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let byte = state.read();
    log::trace!("[microcode] byte read: {:02x}", byte);
    ctl.push(byte);
    OK_CONSUME_CYCLE
}

/// Read `A` register, do not consume the current cycle
pub fn a(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.a());
    OK_PLAY_NEXT_ACTION
}

/// Read `B` register, do not consume the current cycle
pub fn b(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.b());
    OK_PLAY_NEXT_ACTION
}

/// Read `C` register, do not consume the current cycle
pub fn c(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.c());
    OK_PLAY_NEXT_ACTION
}

/// Read `D` register, do not consume the current cycle
pub fn d(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.d());
    OK_PLAY_NEXT_ACTION
}

/// Read `E` register, do not consume the current cycle
pub fn e(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.e());
    OK_PLAY_NEXT_ACTION
}

/// Read `H` register, do not consume the current cycle
pub fn h(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.h());
    OK_PLAY_NEXT_ACTION
}

/// Read `L` register, do not consume the current cycle
pub fn l(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    ctl.push(state.regs.l());
    OK_PLAY_NEXT_ACTION
}

/// Read `BC` register, do not consume the current cycle
pub fn bc(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let bytes = state.regs.bc.to_be_bytes();
    ctl.push(bytes[0]);
    ctl.push(bytes[1]);
    OK_PLAY_NEXT_ACTION
}

/// Read `DE` register, do not consume the current cycle
pub fn de(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let bytes = state.regs.de.to_be_bytes();
    ctl.push(bytes[0]);
    ctl.push(bytes[1]);
    OK_PLAY_NEXT_ACTION
}

/// Read `HL` register, do not consume the current cycle
pub fn hl(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let bytes = state.regs.hl.to_be_bytes();
    ctl.push(bytes[0]);
    ctl.push(bytes[1]);
    OK_PLAY_NEXT_ACTION
}

/// Read `SP` register, do not consume the current cycle
pub fn sp(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let bytes = state.regs.sp.to_be_bytes();
    ctl.push(bytes[0]);
    ctl.push(bytes[1]);
    OK_PLAY_NEXT_ACTION
}

/// Read the byte from the u16 addr stored in the cache, do consume the cycle
pub fn ind(ctl: &mut MicrocodeController, state: &mut State) -> MicrocodeFlow {
    let addr = u16::from_be_bytes([ctl.pop(), ctl.pop()]);
    let value = state.read_bus(addr);
    ctl.push(value);
    OK_CONSUME_CYCLE
}
