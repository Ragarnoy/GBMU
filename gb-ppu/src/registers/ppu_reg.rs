use super::{Control, PalettesMono, Register, RegisterArray, Scrolling, Stat, WindowPos};
use crate::UNDEFINED_VALUE;
use gb_bus::{Address, Error, FileOperation, IORegArea};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPURegisters {
    control: Rc<RefCell<Control>>,
    stat: Rc<RefCell<Stat>>,
    scrolling: Rc<RefCell<Scrolling>>,
    dma: Rc<RefCell<u8>>,
    pal_mono: Rc<RefCell<PalettesMono>>,
    window_pos: Rc<RefCell<WindowPos>>,
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

fn read_register_value_at<R: Register, const S: usize>(
    register: &Rc<RefCell<impl RegisterArray<R, S>>>,
    pos: usize,
) -> u8 {
    match register.try_borrow() {
        Ok(register) => register.read(pos),
        Err(_) => {
            log::error!("failed ppu register array read");
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

fn write_register_value_at<R: Register, const S: usize>(
    register: &Rc<RefCell<impl RegisterArray<R, S>>>,
    pos: usize,
    value: u8,
) {
    match register.try_borrow_mut() {
        Ok(mut register) => register.write(pos, value),
        Err(_) => {
            log::error!("failed ppu register array write");
        }
    }
}

impl FileOperation<IORegArea> for PPURegisters {
    fn read(&self, addr: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
        log::warn!("missing ppu registers read");
        match addr.area_type() {
            IORegArea::Lcd => match addr.get_address() {
                0x00 => Ok(read_register_value(&self.control)),
                0x01 => Ok(read_register_value(&self.stat)),
                pos @ 0x02..=0x05 => Ok(read_register_value_at(&self.scrolling, pos - 0x02)),
                0x06 => Ok(read_register_value(&self.dma)),
                pos @ 0x07..=0x09 => Ok(read_register_value_at(&self.pal_mono, pos - 0x07)),
                pos @ 0x0A..=0x0B => Ok(read_register_value_at(&self.window_pos, pos - 0x0A)),
                _ => Err(Error::SegmentationFault(addr.into())),
            },
            IORegArea::VRamBank => Ok(UNDEFINED_VALUE),
            IORegArea::VramDma => Ok(UNDEFINED_VALUE),
            IORegArea::BgObjPalettes => Ok(UNDEFINED_VALUE),
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }

    fn write(&mut self, v: u8, addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        log::warn!("missing ppu registers write");
        match addr.area_type() {
            IORegArea::Lcd => {
                match addr.get_address() {
                    0x00 => write_register_value(&self.control, v),
                    0x01 => write_register_value(&self.stat, v),
                    pos @ 0x02..=0x05 => write_register_value_at(&self.scrolling, pos - 0x02, v),
                    0x06 => write_register_value(&self.dma, v),
                    pos @ 0x07..=0x09 => write_register_value_at(&self.pal_mono, pos - 0x07, v),
                    pos @ 0x0A..=0x0B => write_register_value_at(&self.window_pos, pos - 0x0A, v),
                    _ => return Err(Error::SegmentationFault(addr.into())),
                };
                Ok(())
            }
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
