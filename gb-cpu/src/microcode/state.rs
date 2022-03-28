use crate::{io_registers::IORegisters, registers::Registers};
use gb_bus::Bus;
use std::{cell::RefCell, rc::Rc};

pub struct State<'a> {
    bus: &'a mut dyn Bus<u8>,
    pub regs: &'a mut Registers,
    pub int_flags: Rc<RefCell<IORegisters>>,
}

impl<'a> State<'a> {
    pub fn new(
        regs: &'a mut Registers,
        bus: &'a mut dyn Bus<u8>,
        int_flags: Rc<RefCell<IORegisters>>,
    ) -> Self {
        Self {
            bus,
            regs,
            int_flags,
        }
    }

    /// Read the byte at the `Program Counter` then increment it
    pub fn read(&mut self) -> u8 {
        let res = self.read_bus(self.regs.pc);
        let (new_pc, overflowing) = self.regs.pc.overflowing_add(1);
        if overflowing {
            panic!("pc should not overflow")
        }
        self.regs.pc = new_pc;
        res
    }

    pub fn read_bus(&self, addr: u16) -> u8 {
        self.bus.read(addr, None).unwrap_or(0xff)
    }

    /// read byte at the addr of `HL`
    pub fn read_hl(&self) -> u8 {
        self.read_bus(self.regs.hl)
    }

    /// write byte at the addr of `HL`
    pub fn write_hl(&mut self, v: u8) {
        self.write_bus(self.regs.hl, v)
    }

    pub fn write_bus(&mut self, addr: u16, v: u8) {
        if let Err(e) = self.bus.write(addr, v, None) {
            log::error!(
                "while writing the value {:02x} in the bus at {:04x} got the error: {}",
                v,
                addr,
                e
            )
        }
    }
}
