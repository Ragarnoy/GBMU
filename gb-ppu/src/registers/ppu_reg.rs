use super::{Control, Register, Stat};
use crate::error::{PPUError, PPUResult};
use crate::UNDEFINED_VALUE;
use gb_bus::{Address, Error, FileOperation, IORegArea};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPURegisters {
    control: Rc<RefCell<Control>>,
    stat: Rc<RefCell<Stat>>,
}

fn read_register_value(register: &Rc<RefCell<impl Register>>) -> u8 {
    match register.try_borrow() {
        Ok(register) => register.read(),
        Err(_) => {
            log::error!("failed ppu register read");
            UNDEFINED_VALUE
        }
    }
}

fn write_register_value(register: &Rc<RefCell<impl Register>>, value: u8) {
    match register.try_borrow_mut() {
        Ok(mut register) => register.write(value),
        Err(_) => {
            log::error!("failed ppu register write");
        }
    }
}

impl FileOperation<IORegArea> for PPURegisters {
    fn read(&self, addr: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
        log::warn!("Unimplemented ppu registers read");
        match addr.area_type() {
            IORegArea::Lcd => match addr.get_address() {
                0 => Ok(read_register_value(&self.control)),
                1 => Ok(read_register_value(&self.stat)),
                _ => Ok(UNDEFINED_VALUE),
            },
            IORegArea::VRamBank => Ok(UNDEFINED_VALUE),
            IORegArea::VramDma => Ok(UNDEFINED_VALUE),
            IORegArea::BgObjPalettes => Ok(UNDEFINED_VALUE),
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }

    fn write(&mut self, v: u8, addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        log::warn!("Unimplemented ppu registers write");
        match addr.area_type() {
            IORegArea::Lcd => {
                match addr.get_address() {
                    0 => write_register_value(&self.control, v),
                    1 => write_register_value(&self.stat, v),
                    _ => {}
                };
                Ok(())
            }
            IORegArea::VRamBank => Ok(()),
            IORegArea::VramDma => Ok(()),
            IORegArea::BgObjPalettes => Ok(()),
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
}
