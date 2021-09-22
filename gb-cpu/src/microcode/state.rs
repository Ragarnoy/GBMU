use crate::registers::Registers;
use gb_bus::Bus;

pub struct State<'a, B: Bus<u8>> {
    bus: &'a mut B,
    regs: &'a mut Registers,
}

impl<'a, B: Bus<u8>> State<'a, B> {
    pub fn new(regs: &'a mut Registers, bus: &'a mut B) -> Self {
        Self { bus, regs }
    }

    /// Read the byte at the `Program Counter` then increment it
    pub fn read(&mut self) -> u8 {
        let res = self.bus.read(self.regs.pc).unwrap_or(0xff);
        self.regs.pc += 1;
        res
    }
}
