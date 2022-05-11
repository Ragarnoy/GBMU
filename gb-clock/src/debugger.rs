/// The interface for the clock's usage of the debugger.
pub trait Debugger<B> {
    /// The debugger checks if a breakpoint has been reached and return true if the execution should stop.
    fn breakpoints(&self, _: &B) -> bool;
}
