pub mod cpu;

use cpu::Cpu;
use crate::memory::Memory;
use crate::memory::area::rom::mbc::Mbc;

#[derive(Debug)]
pub struct Processor {
    clock: u32,
    cpu: Cpu,
    memory: Memory,
}


impl Processor {
    pub fn new(mbc: Mbc, data: Vec<u8>) -> Self {
        Processor {
            clock: 0,
            cpu: Cpu::default(),
            memory: Memory::new(mbc, data)
        }
    }
    pub fn run(&mut self) {
        self.clock = self.cpu.step(&self.memory);
    }
}
