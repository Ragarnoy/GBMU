use crate::registers::Registers;
use gb_bus::Bus;

pub mod opcode;
pub mod opcode_cb;
mod toolkit;

/// Microcode action, their role is to execute one step of an Opcode
/// Each Actions take at most 1 `M-Cycle`
pub type Action<B: Bus<u8>> =
    fn(regs: &mut Registers, bus: &mut B, cache: &mut Vec<u8>) -> Continuum;

/// Continuum allow the action to control the flow of the microcode
pub enum Continuum {
    /// the action perform without error
    Ok,
    /// the action got an error
    Err,
    /// Stop evaluating the remaining Action of the current Opcode
    /// **AND** don't consume a cycle for this Action
    Break,
}
