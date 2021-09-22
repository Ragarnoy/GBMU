use crate::{microcode::controller::MicrocodeController, registers::Registers};
use gb_bus::AddressBus;
use gb_clock::Ticker;

#[derive(Default)]
pub struct Cpu {
    registers: Registers,
    controller: MicrocodeController<AddressBus>,
}

impl Ticker<AddressBus> for Cpu {
    fn cycle_count(&self) -> gb_clock::Tick {
        gb_clock::Tick::MCycle
    }

    fn tick(&mut self, addr_bus: &mut AddressBus) {
        self.controller.step(&mut self.registers, addr_bus)
    }
}
