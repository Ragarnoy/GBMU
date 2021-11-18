use gb_bus::{Area, FileOperation, IORegArea};

#[derive(Default, Debug, Clone, Copy)]
pub struct InterruptFlags {
    pub master_enable: bool,
    pub flag: u8,
    pub enable_mask: u8,
}

impl InterruptFlags {
    pub fn is_interrupt_ready(&self) -> bool {
        if !self.master_enable {
            return false;
        }
        self.flag & self.enable_mask != 0
    }
}

impl FileOperation<Area> for InterruptFlags {
    fn read(&self, _addr: Box<dyn gb_bus::Address<Area>>) -> Result<u8, gb_bus::Error> {
        Ok(self.enable_mask)
    }
    fn write(&mut self, v: u8, _addr: Box<dyn gb_bus::Address<Area>>) -> Result<(), gb_bus::Error> {
        self.enable_mask = v;
        Ok(())
    }
}

impl FileOperation<IORegArea> for InterruptFlags {
    fn read(&self, _addr: Box<dyn gb_bus::Address<IORegArea>>) -> Result<u8, gb_bus::Error> {
        Ok(self.flag)
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
