mod condition;
pub mod controller;
mod dec;
mod fetch;
mod fetch_cb;
mod flag;
mod ident;
mod inc;
mod jump;
pub mod opcode;
pub mod opcode_cb;
mod read;
mod state;
mod write;

pub(crate) use controller::MicrocodeController;
pub(crate) use state::State;

pub type MicrocodeFlow = std::ops::ControlFlow<CycleDigest, CycleDigest>;

/// List the possible behavior for the cycle to be disgested
#[derive(PartialEq, Eq)]
pub enum CycleDigest {
    /// Consume the cycle
    Consume,
    /// Continue executing actions in the same cycle
    Again,
}
