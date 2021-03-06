use std::ops::Not;

use gb_bus::{Address, Area, Error, FileOperation, IORegArea, Source};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Speed {
    Normal,
    Double,
}

impl Not for Speed {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Speed::Double => Speed::Normal,
            Speed::Normal => Speed::Double,
        }
    }
}

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Debug, Clone, Copy)]
pub struct IORegisters {
    pub master_enable: bool,
    pub flag: u8,
    pub enable_mask: u8,

    pub current_speed: Speed,
    pub prepare_to_switch: bool,
}

impl Default for IORegisters {
    fn default() -> Self {
        Self {
            master_enable: false,
            flag: 0,
            enable_mask: 9,
            current_speed: Speed::Normal,
            prepare_to_switch: false,
        }
    }
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
    pub fn need_to_change_speed(&self) -> bool {
        self.prepare_to_switch
    }

    /// Switch the current speed of the cpu
    pub fn switch_speed(&mut self) {
        self.current_speed = !self.current_speed;
        self.prepare_to_switch = false;
    }

    /// Determine if we are in the double mode of the gameboy color
    pub fn fast_mode(&self) -> bool {
        self.current_speed == Speed::Double
    }
}

impl<A> FileOperation<A, Area> for IORegisters
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, _addr: A, _source: Option<Source>) -> Result<u8, gb_bus::Error> {
        Ok(IORegisters::FLAG_MASK | self.enable_mask)
    }

    fn write(&mut self, v: u8, _addr: A, _source: Option<Source>) -> Result<(), gb_bus::Error> {
        self.enable_mask = v & (!IORegisters::FLAG_MASK);
        Ok(())
    }
}

impl<A> FileOperation<A, IORegArea> for IORegisters
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        match addr.area_type() {
            IORegArea::IF => Ok(IORegisters::FLAG_MASK | self.flag),
            IORegArea::Key1 => Ok(double_speed_register(
                self.fast_mode(),
                self.prepare_to_switch,
            )),
            _ => Err(gb_bus::Error::bus_error(addr.into())),
        }
    }

    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), gb_bus::Error> {
        match addr.area_type() {
            IORegArea::IF => self.flag = v & !(IORegisters::FLAG_MASK),
            IORegArea::Key1 => {
                self.prepare_to_switch = v & 1 == 1;
            }
            _ => return Err(gb_bus::Error::bus_error(addr.into())),
        }
        Ok(())
    }
}

/// generate the key1 register from to current & desired speed mode
fn double_speed_register(is_double_speed: bool, prepare_to_switch: bool) -> u8 {
    let mut v = 0;

    if is_double_speed {
        v |= 0x80;
    }
    if prepare_to_switch {
        v |= 0x1;
    }

    v
}

#[test]
fn test_double_speed_regs() {
    assert_eq!(double_speed_register(false, false), 0x00);
    assert_eq!(double_speed_register(true, false), 0x80);
    assert_eq!(double_speed_register(true, true), 0x81);
    assert_eq!(double_speed_register(false, true), 0x01);
}
