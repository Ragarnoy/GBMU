use super::cpu::Cpu;
use crate::memory::Memory;

#[derive(Debug)]
pub struct Processor {
    clock: u32,
    cpu: Cpu,
    memory: Memory,
}

impl Processor {
    pub fn run(&mut self) {
        self.clock = self.cpu.step(&self.memory);
    }
}
