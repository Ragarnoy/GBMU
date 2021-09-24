use crate::registers::Registers;
use gb_bus::Bus;

pub struct State<'a> {
    bus: &'a mut dyn Bus<u8>,
    pub regs: &'a mut Registers,
}

impl<'a> State<'a> {
    pub fn new(regs: &'a mut Registers, bus: &'a mut dyn Bus<u8>) -> Self {
        Self { bus, regs }
    }

    /// Read the byte at the `Program Counter` then increment it
    pub fn read(&mut self) -> u8 {
        let res = self.bus.read(self.regs.pc).unwrap_or(0xff);
        self.regs.pc += 1;
        res
    }
}
