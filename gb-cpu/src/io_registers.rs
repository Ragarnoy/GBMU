use gb_bus::{Area, FileOperation, IORegArea};

#[derive(Default, Debug, Clone, Copy)]
pub struct IORegisters {
    pub master_enable: bool,
    pub flag: u8,
    pub enable_mask: u8,

    #[cfg(feature = "cgb")]
    pub current_speed: Speed,
    #[cfg(feature = "cgb")]
    pub desired_speed: Speed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speed {
    Normal,
    Double,
}

impl Default for Speed {
    fn default() -> Self {
        Self::Normal
    }
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
            #[cfg(feature = "cgb")]
            IORegArea::DoubleSpeed => Ok(double_speed_register(
                self.current_speed,
                self.desired_speed,
            )),
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

/// generate the key1 register from to current & desired speed mode
#[cfg(feature = "cgb")]
fn double_speed_register(current_speed: Speed, desired_speed: Speed) -> u8 {
    let mut v = 0;

    if current_speed == Speed::Double {
        v |= 0x80;
    }
    if desired_speed == Speed::Double {
        v |= 0x1;
    }

    v
}

#[cfg(feature = "cgb")]
#[test]
fn test_double_speed_regs() {
    use Speed::{Double, Normal};

    assert_eq!(double_speed_register(Normal, Normal), 0x00);
    assert_eq!(double_speed_register(Double, Normal), 0x80);
    assert_eq!(double_speed_register(Double, Double), 0x81);
    assert_eq!(double_speed_register(Normal, Double), 0x01);
}

#[cfg(feature = "cgb")]
#[test]
fn test_change_speed() {
    use Speed::{Double, Normal};

    fn new_io_reg_speed(current: Speed, desired: Speed) -> IORegisters {
        IORegisters {
            current_speed: current,
            desired_speed: desired,
            ..Default::default()
        }
    }

    assert!(!new_io_reg_speed(Normal, Normal).need_to_change_speed());
    assert!(!new_io_reg_speed(Double, Double).need_to_change_speed());
    assert!(new_io_reg_speed(Normal, Double).need_to_change_speed());
    assert!(new_io_reg_speed(Double, Normal).need_to_change_speed());
}
