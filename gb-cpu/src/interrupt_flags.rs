use gb_bus::{Addr, Area, FileOperation, IORegArea};

#[derive(Default, Debug, Clone, Copy)]
pub struct InterruptFlags {
    pub master_enable: bool,
    pub flag: u8,
    pub enable_mask: u8,
}

impl InterruptFlags {
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

impl FileOperation<Addr<Area>, Area> for InterruptFlags {
    fn read(&self, _addr: Addr<Area>) -> Result<u8, gb_bus::Error> {
        Ok(self.enable_mask)
    }

    fn write(&mut self, v: u8, _addr: Addr<Area>) -> Result<(), gb_bus::Error> {
        self.enable_mask = v;
        Ok(())
    }
}

impl FileOperation<Addr<IORegArea>, IORegArea> for InterruptFlags {
    fn read(&self, _addr: Addr<IORegArea>) -> Result<u8, gb_bus::Error> {
        Ok(self.flag)
    }

    fn write(&mut self, v: u8, _addr: Addr<IORegArea>) -> Result<(), gb_bus::Error> {
        self.flag = v;
        Ok(())
    }
}
