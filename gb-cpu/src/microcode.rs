mod arithmetic;
mod bitwise;
mod condition;
pub mod controller;
mod dec;
mod fetch;
mod fetch_cb;
mod flag;
mod inc;
mod interrupts;
mod jump;
mod logic;
mod math;
pub mod opcode;
pub mod opcode_cb;
mod push;
mod read;
mod state;
mod utils;
mod write;

pub(crate) use controller::MicrocodeController;
pub(crate) use state::State;

pub type MicrocodeFlow = std::ops::ControlFlow<(), ()>;
pub const CONTINUE: MicrocodeFlow = MicrocodeFlow::Continue(());
pub const BREAK: MicrocodeFlow = MicrocodeFlow::Break(());
