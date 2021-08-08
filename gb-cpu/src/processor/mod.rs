pub mod cpu;

use std::fs::File;

use cpu::Cpu;
use crate::memory::Memory;

#[derive(Debug)]
pub struct Processor {
    clock: u32,
    cpu: Cpu,
    memory: Memory,
}


impl Processor {
    pub fn new(bios: File, cartridge: File) -> Self {
        Processor {
            clock: 0,
            cpu: Cpu::default(),
            memory: Memory::new(bios, cartridge)
        }
    }
    pub fn run(&mut self) {
        self.clock = self.cpu.step(&self.memory);
    }
}
