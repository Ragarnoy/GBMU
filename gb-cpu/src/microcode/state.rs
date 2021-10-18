use crate::registers::Registers;
use gb_bus::{constant::IE_REG_START, io_reg_constant::INTERRUPT_FLAG, Bus};

pub struct State<'a> {
    bus: &'a mut dyn Bus<u8>,
    pub regs: &'a mut Registers,
}

impl<'a> State<'a> {
    pub fn new(regs: &'a mut Registers, bus: &'a mut dyn Bus<u8>) -> Self {
        Self { bus, regs }
    }

    /// Read the byte at the `Program Counter` then increment it
    pub fn read(&mut self) -> u8 {
        let res = self.read_bus(self.regs.pc);
        self.regs.pc += 1;
        res
    }

    pub fn read_bus(&self, addr: u16) -> u8 {
        self.bus.read(addr).unwrap_or(0xff)
    }

    /// read byte at the addr of `HL`
    pub fn read_hl(&self) -> u8 {
        self.read_bus(self.regs.hl)
    }

    /// read byte at interrupt flag register's address
    pub fn read_interrupt_flag(&self) -> u8 {
        self.read_bus(INTERRUPT_FLAG)
    }

    /// read byte at interrupt enable register's address
    pub fn read_interrupt_enable(&self) -> u8 {
        self.read_bus(IE_REG_START)
    }

    /// write byte at the addr of `HL`
    pub fn write_hl(&mut self, v: u8) {
        self.write_bus(self.regs.hl, v)
    }

    pub fn write_bus(&mut self, addr: u16, v: u8) {
        if let Err(e) = self.bus.write(addr, v) {
            log::error!(
                "while writing the value {:x} in the bus at {:x} got the error: {:?}",
                v,
                addr,
                e
            )
        }
    }

    /// write byte at interrupt flag register's address
    pub fn write_interrupt_flag(&mut self, v: u8) {
        self.write_bus(INTERRUPT_FLAG, v)
    }

    /// write byte at interrupt enable register's address
    pub fn write_interrupt_enable(&mut self, v: u8) {
        self.write_bus(IE_REG_START, v)
    }
}
