use crate::{cpu::Cpu, io_registers::IORegisters};
use std::{cell::RefCell, rc::Rc};

pub fn new_cpu() -> (Cpu, Rc<RefCell<IORegisters>>) {
    let cpu = Cpu::default();
    let flags = cpu.io_regs.clone();

    (cpu, flags)
}
