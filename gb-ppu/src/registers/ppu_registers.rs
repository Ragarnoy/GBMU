use super::LcdReg;
use crate::error::{PPUError, PPUResult};
use crate::UNDEFINED_VALUE;
use gb_bus::{Address, Error, FileOperation, IORegArea};
use std::cell::RefCell;
use std::rc::Rc;

/// Allow external structures to read/write the registers of the ppu.
///
/// The read/write operation might be ignored if the ppu is currently using the concerned memory area, but unlike for the memory, this should not happen.
pub struct PPURegisters {
    lcd: Rc<RefCell<LcdReg>>,
}

impl PPURegisters {
    /// Build a PPURegisters from references counters of Vram and Oam.
    ///
    /// This function is used by [Ppu.registers()](crate::Ppu::registers), you should not need to call this constructor yourself.
    pub fn new(lcd: Rc<RefCell<LcdReg>>) -> Self {
        PPURegisters { lcd }
    }

    /// Completely replace the lcd registers of the ppu.
    ///
    /// This function exist for debugging purpose.
    pub fn overwrite_lcd(&self, data: [u8; LcdReg::SIZE]) -> PPUResult<()> {
        match self.lcd.try_borrow_mut() {
            Ok(mut lcd) => {
                *lcd = data.into();
                Ok(())
            }
            Err(_) => Err(PPUError::RegistersUnavailable {
                reg_name: String::from("lcd"),
            }),
        }
    }
}

impl FileOperation<IORegArea> for PPURegisters {
    fn read(&self, addr: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
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

#[cfg(test)]
mod read {
    use super::{LcdReg, PPURegisters};
    use crate::test_tools::TestIORegAddress;
    use gb_bus::FileOperation;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn lcd_control() {
        let data: [u8; LcdReg::SIZE] = [0x42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let lcd = Rc::new(RefCell::new(data.into()));
        let ppu_reg = PPURegisters::new(lcd);

        let res = ppu_reg
            .read(Box::new(TestIORegAddress::control()))
            .expect("Try reading value from lcd control");
        assert_eq!(res, 0x42, "invalid value from lcd control");
    }

    #[test]
    fn lcd_dma() {
        let data: [u8; LcdReg::SIZE] = [0, 0x42, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let lcd = Rc::new(RefCell::new(data.into()));
        let ppu_reg = PPURegisters::new(lcd);

        let res = ppu_reg
            .read(Box::new(TestIORegAddress::stat()))
            .expect("Try reading value from lcd dma");
        assert_eq!(res, 0x42, "invalid value from lcd dma");
    }

    #[test]
    fn lcd_window_pos() {
        let data: [u8; LcdReg::SIZE] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x42];
        let lcd = Rc::new(RefCell::new(data.into()));
        let ppu_reg = PPURegisters::new(lcd);

        let res = ppu_reg
            .read(Box::new(TestIORegAddress::window_pos(1)))
            .expect("Try reading value from lcd window_pos");
        assert_eq!(res, 0x42, "invalid value from lcd window_pos");
    }
}

#[cfg(test)]
mod write {
    use super::{LcdReg, PPURegisters};
    use crate::test_tools::TestIORegAddress;
    use gb_bus::FileOperation;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn lcd_stat() {
        let data: [u8; LcdReg::SIZE] = [0; LcdReg::SIZE];
        let lcd = Rc::new(RefCell::new(data.into()));
        let mut ppu_reg = PPURegisters::new(lcd);

        ppu_reg
            .write(0b1111_1111, Box::new(TestIORegAddress::stat()))
            .expect("Try write value into lcd stat");
        let res = ppu_reg
            .read(Box::new(TestIORegAddress::stat()))
            .expect("Try reading value from lcd stat");
        assert_eq!(res, 0b0111_1000, "invalid value from lcd stat");
    }

    #[test]
    fn lcd_palette() {
        let data: [u8; LcdReg::SIZE] = [0; LcdReg::SIZE];
        let lcd = Rc::new(RefCell::new(data.into()));
        let mut ppu_reg = PPURegisters::new(lcd);

        ppu_reg
            .write(0x42, Box::new(TestIORegAddress::palette(2)))
            .expect("Try write value into lcd palette");
        let res = ppu_reg
            .read(Box::new(TestIORegAddress::palette(2)))
            .expect("Try reading value from lcd palette");
        assert_eq!(res, 0x42, "invalid value from lcd palette");
    }

    #[test]
    fn lcd_scrolling() {
        let data: [u8; LcdReg::SIZE] = [0; LcdReg::SIZE];
        let lcd = Rc::new(RefCell::new(data.into()));
        let mut ppu_reg = PPURegisters::new(lcd);

        for pos in 0..4 {
            ppu_reg
                .write(0x42, Box::new(TestIORegAddress::scrolling(pos)))
                .expect("Try write value into lcd scrolling");
        }
        let res = ppu_reg
            .read(Box::new(TestIORegAddress::scrolling(0)))
            .expect("Try reading value from lcd scrolling");
        assert_eq!(res, 0x42, "invalid value from lcd scrolling");
        let res = ppu_reg
            .read(Box::new(TestIORegAddress::scrolling(1)))
            .expect("Try reading value from lcd scrolling");
        assert_eq!(res, 0x42, "invalid value from lcd scrolling");
        let res = ppu_reg
            .read(Box::new(TestIORegAddress::scrolling(2)))
            .expect("Try reading value from lcd scrolling");
        assert_eq!(res, 0x00, "invalid value from lcd scrolling");
        let res = ppu_reg
            .read(Box::new(TestIORegAddress::scrolling(3)))
            .expect("Try reading value from lcd scrolling");
        assert_eq!(res, 0x42, "invalid value from lcd scrolling");
    }
}
