use gb_bus::{Area, FileOperation, IORegArea};

#[derive(Default, Debug, Clone, Copy)]
pub struct InterruptFlags {
    pub master_enable: bool,
    pub flag: u8,
    pub enable_mask: u8,
}

impl InterruptFlags {
    const FLAG_MASK: u8 = 0b1110_0000;

    pub fn is_interrupt_ready(&self) -> bool {
        self.flag & self.enable_mask != 0
    }

    pub fn should_handle_interrupt(&self) -> bool {
        self.master_enable
    }

    pub fn interrupt_to_handle(&self) -> bool {
        self.should_handle_interrupt() && self.is_interrupt_ready()
    }
}

impl FileOperation<Area> for InterruptFlags {
    fn read(&self, _addr: Box<dyn gb_bus::Address<Area>>) -> Result<u8, gb_bus::Error> {
        Ok(InterruptFlags::FLAG_MASK | self.enable_mask)
    }

    fn write(&mut self, v: u8, _addr: Box<dyn gb_bus::Address<Area>>) -> Result<(), gb_bus::Error> {
        self.enable_mask = v & (!InterruptFlags::FLAG_MASK);
        Ok(())
    }
}

impl FileOperation<IORegArea> for InterruptFlags {
    fn read(&self, _addr: Box<dyn gb_bus::Address<IORegArea>>) -> Result<u8, gb_bus::Error> {
        Ok(InterruptFlags::FLAG_MASK | self.flag)
    }

    fn write(
        &mut self,
        v: u8,
        _addr: Box<dyn gb_bus::Address<IORegArea>>,
    ) -> Result<(), gb_bus::Error> {
        self.flag = v & (!InterruptFlags::FLAG_MASK);
        Ok(())
    }
}
