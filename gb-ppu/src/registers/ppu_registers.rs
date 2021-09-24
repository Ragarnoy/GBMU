use super::LcdReg;
use crate::UNDEFINED_VALUE;
use gb_bus::{Address, Error, FileOperation, IORegArea};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPURegisters {
    lcd: Rc<RefCell<LcdReg>>,
}

impl PPURegisters {
    pub fn new(lcd: Rc<RefCell<LcdReg>>) -> Self {
        PPURegisters { lcd }
    }

    /// Completely replace the lcd registers of the ppu.
    ///
    /// This function exist for debugging purpose.
    pub fn overwrite_lcd(&mut self, data: [u8; LcdReg::SIZE]) {
        match self.lcd.try_borrow_mut() {
            Ok(mut lcd) => *lcd = data.into(),
            Err(_) => log::warn!("failed ppu Lcd register read"),
        }
    }
}

impl FileOperation<IORegArea> for PPURegisters {
    fn read(&self, addr: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
        log::warn!("missing ppu registers read");
        match addr.area_type() {
            IORegArea::Lcd => match self.lcd.try_borrow() {
                Ok(lcd) => lcd.read(addr),
                Err(_) => {
                    log::warn!("failed ppu Lcd register read");
                    Ok(UNDEFINED_VALUE)
                }
            },
            IORegArea::VRamBank => {
                log::warn!("missing ppu VramBank register");
                Ok(UNDEFINED_VALUE)
            }
            IORegArea::VramDma => {
                log::warn!("missing ppu VramDma register");
                Ok(UNDEFINED_VALUE)
            }
            IORegArea::BgObjPalettes => {
                log::warn!("missing ppu BgObjPalette register");
                Ok(UNDEFINED_VALUE)
            }
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }

    fn write(&mut self, v: u8, addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        log::warn!("missing ppu registers write");
        match addr.area_type() {
            IORegArea::Lcd => match self.lcd.try_borrow_mut() {
                Ok(mut lcd) => lcd.write(addr, v),
                Err(_) => {
                    log::warn!("failed ppu register write");
                    Ok(())
                }
            },
            IORegArea::VRamBank => {
                log::warn!("missing ppu VRamBank registers write");
                Ok(())
            }
            IORegArea::VramDma => {
                log::warn!("missing ppu VRamDma registers write");
                Ok(())
            }
            IORegArea::BgObjPalettes => {
                log::warn!("missing ppu BgObjPalettes registers write");
                Ok(())
            }
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
}
