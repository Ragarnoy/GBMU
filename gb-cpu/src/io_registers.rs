use gb_bus::{Address, Area, Error, FileOperation, IORegArea};

#[derive(Default, Debug, Clone, Copy)]
pub struct IORegisters {
    pub master_enable: bool,
    pub flag: u8,
    pub enable_mask: u8,

    #[cfg(feature = "cgb")]
    pub current_speed: bool,
    #[cfg(feature = "cgb")]
    pub desired_speed: bool,
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
    #[cfg(feature = "cgb")]
    pub fn need_to_change_speed(&self) -> bool {
        self.current_speed != self.desired_speed
    }
}

impl<A> FileOperation<A, Area> for IORegisters
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, _addr: A) -> Result<u8, Error> {
        Ok(self.enable_mask)
    }

    fn write(&mut self, v: u8, _addr: A) -> Result<(), Error> {
        self.enable_mask = v;
        Ok(())
    }
}

impl<A> FileOperation<A, IORegArea> for IORegisters
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, addr: A) -> Result<u8, Error> {
        match addr.area_type() {
            IORegArea::InterruptFlag => Ok(self.flag),
            #[cfg(feature = "cgb")]
            IORegArea::DoubleSpeed => Ok(double_speed_register(
                self.current_speed,
                self.desired_speed,
            )),
            _ => Err(gb_bus::Error::bus_error(addr)),
        }
    }

    fn write(&mut self, v: u8, _addr: A) -> Result<(), Error> {
        self.flag = v;
        Ok(())
    }
}

/// generate the key1 register from to current & desired speed mode
#[cfg(feature = "cgb")]
fn double_speed_register(current_speed: bool, desired_speed: bool) -> u8 {
    let mut v = 0;

    if current_speed {
        v |= 0x80;
    }
    if desired_speed {
        v |= 0x1;
    }

    v
}

#[cfg(feature = "cgb")]
#[test]
fn test_double_speed_regs() {
    assert_eq!(double_speed_register(false, false), 0x00);
    assert_eq!(double_speed_register(true, false), 0x80);
    assert_eq!(double_speed_register(true, true), 0x81);
    assert_eq!(double_speed_register(false, true), 0x01);
}
