use crate::registers::Registers;
use gb_bus::{AddressBus, Bus};
use gb_clock::Ticker;
use gb_roms::{Opcode, OpcodeGenerator};

#[derive(Default)]
pub struct Cpu {
    registers: Registers,
    current_action: Option<CpuAction>,
}

struct CpuAction {
    opcode: Opcode,
    timeout_exec: usize,
}

impl CpuAction {
    fn new(opcode: Opcode) -> Self {
        use crate::timing::Timing;

        let timeout_exec = opcode.timing();
        Self {
            timeout_exec,
            opcode,
        }
    }
}

impl Cpu {
    fn execute_opcode(&mut self, opc: Opcode, _addr_bus: &mut AddressBus) {
        log::error!("Unhandled opcode: {:?}", opc);
    }

    fn retrieve_next_opcode(&mut self, addr_bus: &mut AddressBus) {
        let it = CpuIterator::new(self, addr_bus);
        let mut gen = OpcodeGenerator::new(it);
        if let Some(next) = gen.next() {
            next.map_or_else(
                |e| log::warn!("OpcodeGenerator returned an error: {:?}", e),
                |opc| self.current_action = Some(CpuAction::new(opc)),
            );
        } else {
            log::trace!("OpcodeGenerator returned nothing");
        }
    }
}

impl Ticker<AddressBus> for Cpu {
    fn cycle_count(&self) -> gb_clock::Tick {
        gb_clock::Tick::MCycle
    }

    fn tick(&mut self, addr_bus: &mut AddressBus) {
        if let Some(action) = self.current_action.as_mut() {
            if action.timeout_exec == 0 {
                let opc = action.opcode.clone();
                self.execute_opcode(opc, addr_bus);
                self.current_action = None;
            } else {
                action.timeout_exec -= 0;
            }
        } else {
            self.retrieve_next_opcode(addr_bus);
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
            Some,
        )
    }
}
