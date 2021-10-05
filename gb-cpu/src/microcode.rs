mod arithmetic;
mod condition;
pub mod controller;
mod dec;
mod fetch;
mod fetch_cb;
mod flag;
mod ident;
mod inc;
mod jump;
mod logic;
mod math;
pub mod opcode;
pub mod opcode_cb;
mod read;
mod state;
mod write;

pub(crate) use controller::MicrocodeController;
pub(crate) use state::State;

pub type MicrocodeFlow = std::ops::ControlFlow<CycleDigest, CycleDigest>;
pub const OK_CONSUME_CYCLE: MicrocodeFlow = MicrocodeFlow::Continue(CycleDigest::Consume);
pub const OK_PLAY_NEXT_ACTION: MicrocodeFlow = MicrocodeFlow::Continue(CycleDigest::Again);

/// List the possible behavior for the cycle to be disgested
#[derive(PartialEq, Eq)]
pub enum CycleDigest {
    /// Consume the cycle
    Consume,
    /// Continue executing actions in the same cycle
    Again,
}
