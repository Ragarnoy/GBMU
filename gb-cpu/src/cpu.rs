use crate::{
    io_registers::IORegisters, microcode::controller::MicrocodeController, registers::Registers,
};
use gb_bus::Bus;
use gb_clock::Ticker;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    pub controller: MicrocodeController,
    pub io_regs: Rc<RefCell<IORegisters>>,
    pub halted_dma: bool,
}

impl Cpu {
    pub fn new(cgb_mode: bool) -> Self {
        Cpu {
            registers: Registers::default(),
            controller: MicrocodeController::new(cgb_mode),
            io_regs: Rc::new(RefCell::new(IORegisters::default())),
            halted_dma: false,
        }
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
        if self.halted_dma {
            return;
        }
        self.controller
            .step(self.io_regs.clone(), &mut self.registers, addr_bus)
    }
}
