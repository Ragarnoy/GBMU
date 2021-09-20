pub mod controller;
mod fetch;
mod fetch_cb;
pub mod opcode;
pub mod opcode_cb;
mod state;

pub(crate) use controller::MicrocodeController;
pub(crate) use state::State;

/// Continuum allow the action to control the flow of the microcode
pub enum Continuum {
    /// the action perform without error
    Ok,
    /// the action got an error
    Err,
    /// Stop evaluating the remaining Action of the current Opcode
    /// **AND** don't consume a cycle for this Action
    Break,
    /// Imediatly execute the next step during the same cycle
    Chain,
}
