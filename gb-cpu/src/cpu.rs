use crate::{
    interrupt_flags::InterruptFlags, microcode::controller::MicrocodeController,
    registers::Registers,
};
use gb_bus::Bus;
use gb_clock::Ticker;
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Debug, Clone)]
pub struct Cpu {
    pub registers: Registers,
    pub controller: MicrocodeController,
    interrupt_flags: Rc<RefCell<InterruptFlags>>,
}

impl Cpu {
    pub fn interrupt_flags(&self) -> Rc<RefCell<InterruptFlags>> {
        self.interrupt_flags.clone()
    }

    pub fn set_registers(&mut self, regs: Registers) {
        self.registers = regs;
    }
}

impl Ticker for Cpu {
    fn cycle_count(&self) -> gb_clock::Tick {
        gb_clock::Tick::MCycle
    }

    fn tick(&mut self, addr_bus: &mut dyn Bus<u8>) {
        self.controller
            .step(self.interrupt_flags.clone(), &mut self.registers, addr_bus)
    }
}
