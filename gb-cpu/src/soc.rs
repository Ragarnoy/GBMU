pub mod cpu;

use crate::memory::Memory;
use cpu::Cpu;

pub struct SOC {
    clock: u32,
    cpu: Cpu,
    memory: Memory,
}

impl Default for SOC {
    fn default() -> Self {
        SOC::new()
    }
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
        match self.cpu.step(&self.memory) {
            Ok(cycles) => self.clock += cycles,
            Err(_) => (),
        }
    }
}
