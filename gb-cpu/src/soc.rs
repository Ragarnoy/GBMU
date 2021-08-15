pub mod cpu;

use crate::memory::Memory;
use cpu::Cpu;

pub struct SOC {
    clock: u32,
    cpu: Cpu,
    memory: Memory,
}

impl SOC {
    pub fn new() -> Self {
        SOC {
            clock: 0,
            cpu: Cpu::default(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        if let Ok(cycles) = self.cpu.step(&self.memory){
            self.clock += cycles;
        }
    }
}
