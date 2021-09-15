use crate::registers::Registers;
use gb_bus::{AddressBus, Bus};
use gb_clock::Ticker;
use gb_roms::{Opcode, OpcodeGenerator};

#[derive(Default)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    fn execute_opcode(&mut self, opc: Opcode, _addr_bus: &mut AddressBus) {
        match opc {
            _ => log::error!("Unhandled opcode: {:?}", opc),
        }
    }
}

impl Ticker<AddressBus> for Cpu {
    fn cycle_count(&self) -> gb_clock::Tick {
        gb_clock::Tick::MCycle
    }

    fn tick(&mut self, addr_bus: &mut AddressBus) {
        let it = CpuIterator::new(self, addr_bus);
        let mut gen = OpcodeGenerator::new(it);
        if let Some(next) = gen.next() {
            next.map_or_else(
                |e| log::warn!("OpcodeGenerator returned an error: {:?}", e),
                |opc| self.execute_opcode(opc, addr_bus),
            );
        } else {
            log::trace!("OpcodeGenerator returned nothing");
        }
    }
}

struct CpuIterator<'a> {
    cpu: &'a mut Cpu,
    addr_bus: &'a mut AddressBus,
}

impl<'a> CpuIterator<'a> {
    fn new(cpu: &'a mut Cpu, addr_bus: &'a mut AddressBus) -> Self {
        CpuIterator { cpu, addr_bus }
    }
}

impl<'a> Iterator for CpuIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let pc = self.cpu.registers.pc;
        let res: Result<u8, gb_bus::Error> = self.addr_bus.read(pc);
        self.cpu.registers.pc += 1;
        res.map_or_else(
            |e| {
                log::warn!("address bus error: {:?}", e);
                None
            },
            |byte| Some(byte),
        )
    }
}
