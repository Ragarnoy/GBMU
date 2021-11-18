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
    fn new(interrupt_flags: Rc<RefCell<InterruptFlags>>) -> Self {
        Self {
            registers: Registers::default(),
            controller: MicrocodeController::default(),
            interrupt_flags,
        }
    }

    pub fn interrupt_flagss(&self) -> Rc<RefCell<InterruptFlags>> {
        self.interrupt_flags.clone()
    }
}

impl Ticker for Cpu {
    fn cycle_count(&self) -> gb_clock::Tick {
        gb_clock::Tick::MCycle
    }

    fn tick(&mut self, addr_bus: &mut dyn Bus<u8>) {
        use std::ops::DerefMut;

        self.controller.step(
            self.interrupt_flags.borrow_mut().deref_mut(),
            &mut self.registers,
            addr_bus,
        )
    }
}
