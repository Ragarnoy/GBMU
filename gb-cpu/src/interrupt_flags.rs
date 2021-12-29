use gb_bus::{Address, Area, FileOperation, IORegArea};

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

impl<A> FileOperation<A, Area> for InterruptFlags
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, _addr: A) -> Result<u8, gb_bus::Error> {
        Ok(self.enable_mask)
    }

    fn write(&mut self, v: u8, _addr: A) -> Result<(), gb_bus::Error> {
        self.enable_mask = v;
        Ok(())
    }
}

impl<A> FileOperation<A, IORegArea> for InterruptFlags
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, _addr: A) -> Result<u8, gb_bus::Error> {
        Ok(self.flag)
    }

    fn write(&mut self, v: u8, _addr: A) -> Result<(), gb_bus::Error> {
        self.flag = v;
        Ok(())
    }
}
