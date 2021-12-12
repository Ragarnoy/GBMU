use gb_bus::{Area, FileOperation, IORegArea};

#[derive(Default, Debug, Clone, Copy)]
pub struct IORegisters {
    pub master_enable: bool,
    pub flag: u8,
    pub enable_mask: u8,

    pub double_speed: bool,
    pub desire_double_speed: bool,
}

impl IORegisters {
    pub fn is_interrupt_ready(&self) -> bool {
        self.flag & self.enable_mask != 0
    }

    pub fn should_handle_interrupt(&self) -> bool {
        self.master_enable
    }

    pub fn interrupt_to_handle(&self) -> bool {
        self.should_handle_interrupt() && self.is_interrupt_ready()
    }

    /// Indicate when we need to switch between `normal speed <=> double speed`
    pub fn need_to_change_speed(&self) -> bool {
        self.double_speed != self.desire_double_speed
    }
}

impl FileOperation<Area> for IORegisters {
    fn read(&self, _addr: Box<dyn gb_bus::Address<Area>>) -> Result<u8, gb_bus::Error> {
        Ok(self.enable_mask)
    }

    fn write(&mut self, v: u8, _addr: Box<dyn gb_bus::Address<Area>>) -> Result<(), gb_bus::Error> {
        self.enable_mask = v;
        Ok(())
    }
}

impl FileOperation<IORegArea> for IORegisters {
    fn read(&self, addr: Box<dyn gb_bus::Address<IORegArea>>) -> Result<u8, gb_bus::Error> {
        match addr.area_type() {
            IORegArea::InterruptFlag => Ok(self.flag),
            _ => Err(gb_bus::Error::bus_error(addr)),
        }
    }

    fn write(
        &mut self,
        v: u8,
        _addr: Box<dyn gb_bus::Address<IORegArea>>,
    ) -> Result<(), gb_bus::Error> {
        self.flag = v;
        Ok(())
    }
}
