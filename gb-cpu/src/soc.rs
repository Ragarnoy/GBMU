pub mod cpu;

use crate::memory::area::rom::mbc::Mbc;
use crate::memory::Memory;
use cpu::Cpu;

pub struct SOC {
    clock: u32,
    cpu: Cpu,
    memory: Memory,
}

impl SOC {
    pub fn new(mbc: Mbc, data: Vec<u8>) -> Self {
        SOC {
            clock: 0,
            cpu: Cpu::default(),
            memory: Memory::new(mbc, data),
        }
    }
    pub fn run(&mut self) {
        self.clock = self.cpu.step(&self.memory);
    }
}
