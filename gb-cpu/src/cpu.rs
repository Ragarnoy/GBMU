use crate::registers::Registers;
use gb_bus::AddressBus;
use gb_clock::Ticker;
use gb_roms::{Opcode, OpcodeGenerator};

pub struct Cpu {
    _registers: Registers,
}

impl Cpu {
    fn execute_opcode(&mut self, opc: Opcode, addr_bus: &mut AddressBus) {}
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
    cpu: &'a Cpu,
    addr_bus: &'a AddressBus,
}

impl<'a> CpuIterator<'a> {
    fn new(cpu: &'a Cpu, addr_bus: &'a AddressBus) -> Self {
        CpuIterator { cpu, addr_bus }
    }
}

impl<'a> Iterator for CpuIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(42)
    }
}
