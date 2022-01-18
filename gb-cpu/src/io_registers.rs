use gb_bus::{Address, Area, Error, FileOperation, IORegArea};

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
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
    const FLAG_MASK: u8 = 0b1110_0000;

    /// Some interrupts are pending (`IE & IF != 0`)
    pub fn is_interrupt_ready(&self) -> bool {
        self.flag & self.enable_mask != 0
    }

    /// Check if `IME` is enabled
    pub fn should_handle_interrupt(&self) -> bool {
        self.master_enable
    }

    /// Check if you have to handle interrupts.
    /// It will check the `IME` and `IE & IF != 0`
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
    fn read(&self, _addr: A) -> Result<u8, gb_bus::Error> {
        Ok(IORegisters::FLAG_MASK | self.enable_mask)
    }

    fn write(&mut self, v: u8, _addr: A) -> Result<(), gb_bus::Error> {
        self.enable_mask = v & (!IORegisters::FLAG_MASK);
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
            IORegArea::IF => Ok(IORegisters::FLAG_MASK | self.flag),
            #[cfg(feature = "cgb")]
            IORegArea::Key1 => Ok(double_speed_register(
                self.current_speed,
                self.desired_speed,
            )),
            _ => Err(gb_bus::Error::bus_error(addr.into())),
        }
    }

    fn write(&mut self, v: u8, addr: A) -> Result<(), gb_bus::Error> {
        match addr.area_type() {
            IORegArea::IF => self.flag = v & !(IORegisters::FLAG_MASK),
            #[cfg(feature = "cgb")]
            IORegArea::Key1 => self.desired_speed = v & 1 == 1,
            _ => return Err(gb_bus::Error::bus_error(addr.into())),
        }
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
