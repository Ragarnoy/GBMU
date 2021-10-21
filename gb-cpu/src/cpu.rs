use crate::{microcode::controller::MicrocodeController, registers::Registers};
use gb_bus::Bus;
use gb_clock::Ticker;

#[derive(Default, Debug, Clone)]
pub struct Cpu {
    pub registers: Registers,
    pub controller: MicrocodeController,
}

impl Ticker for Cpu {
    fn cycle_count(&self) -> gb_clock::Tick {
        gb_clock::Tick::MCycle
    }

    fn tick<B: Bus<u8>>(&mut self, addr_bus: &mut B) {
        self.controller.step(&mut self.registers, addr_bus)
    }
}
