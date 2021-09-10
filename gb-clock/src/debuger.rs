use gb_bus::Bus;

/// The interface for the clock's usage of the debuger.
pub trait Debuger<B: Bus<u8> + Bus<u16>> {
    /// The debuger checks if a breakpoint has been reached and return true if the execution should stop.
    fn breakpoints(&self, adr_bus: &B) -> bool;
}
