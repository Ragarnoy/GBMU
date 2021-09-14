/// The interface for the clock's usage of the debuger.
pub trait Debuger<B> {
    /// The debuger checks if a breakpoint has been reached and return true if the execution should stop.
    fn breakpoints(&self, _: &B) -> bool;
}
