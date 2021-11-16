#[cfg(feature = "registers_logs")]
use crate::interfaces::{Read8BitsReg, Read8BitsRegExt};
use crate::registers::Registers;
#[cfg(feature = "registers_logs")]
use core::fmt::{self, Debug};
use gb_bus::Bus;

#[cfg(feature = "registers_logs")]
impl<'a> Debug for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})\n",
            self.regs.a(),
            self.regs.f(),
            self.regs.b(),
            self.regs.c(),
            self.regs.d(),
            self.regs.e(),
            self.regs.h(),
            self.regs.l(),
            self.regs.sp,
            self.regs.pc,
            self.read_bus(self.regs.pc),
            self.read_bus(self.regs.pc + 1),
            self.read_bus(self.regs.pc + 2),
            self.read_bus(self.regs.pc + 3),
        )
    }
}
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
        log::trace!("[micrococe] reading bus at {:x}", addr);
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
                "while writing the value {:x} in the bus at {:x} got the error: {:?}",
                v,
                addr,
                e
            )
        }
    }
}
