use crate::{cpu::Cpu, io_registers::IORegisters};
use std::{cell::RefCell, rc::Rc};

pub fn new_cpu(cgb_mode: bool) -> (Cpu, Rc<RefCell<IORegisters>>) {
    let cpu = Cpu::new(cgb_mode);
    let flags = cpu.io_regs.clone();

    (cpu, flags)
}
