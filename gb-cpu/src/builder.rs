use crate::{cpu::Cpu, interrupt_flags::InterruptFlags};
use std::{cell::RefCell, rc::Rc};

pub fn new_cpu() -> (Cpu, Rc<RefCell<InterruptFlags>>) {
    let cpu = Cpu::default();

    (cpu, cpu.interrupt_flagss())
}
