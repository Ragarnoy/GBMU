use crate::{microcode::controller::MicrocodeController, registers::Registers};
use gb_bus::{Area, Bus, FileOperation, IORegArea};
use gb_clock::Ticker;

#[derive(Default, Debug, Clone)]
pub struct Cpu {
    pub registers: Registers,
    pub controller: MicrocodeController,
}

impl Ticker for Cpu {
    fn cycle_count(&self) -> gb_clock::Tick {
        gb_clock::Tick::MCycle
    }

    fn tick<B: Bus<u8>>(&mut self, addr_bus: &mut B) {
        self.controller.step(&mut self.registers, addr_bus)
    }
}

impl FileOperation<Area> for Cpu {
    fn read(&self, _addr: Box<dyn gb_bus::Address<Area>>) -> Result<u8, gb_bus::Error> {
        Ok(self.controller.interrupt_enable)
    }
    fn write(&mut self, v: u8, _addr: Box<dyn gb_bus::Address<Area>>) -> Result<(), gb_bus::Error> {
        self.controller.interrupt_enable = v;
        Ok(())
    }
}

impl FileOperation<IORegArea> for Cpu {
    fn read(&self, _addr: Box<dyn gb_bus::Address<IORegArea>>) -> Result<u8, gb_bus::Error> {
        Ok(self.controller.interrupt_flag)
    }
    fn write(
        &mut self,
        v: u8,
        _addr: Box<dyn gb_bus::Address<IORegArea>>,
    ) -> Result<(), gb_bus::Error> {
        self.controller.interrupt_flag = v;
        Ok(())
    }
}
