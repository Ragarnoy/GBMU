use super::{Oam, Vram};
use crate::error::{PPUError, PPUResult};
use gb_bus::{Address, Area, Error, FileOperation};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPUMem {
    vram: Rc<RefCell<Vram>>,
    oam: Rc<RefCell<Oam>>,
}

impl PPUMem {
    pub fn new(vram: Rc<RefCell<Vram>>, oam: Rc<RefCell<Oam>>) -> Self {
        PPUMem { vram, oam }
    }

    pub fn overwrite_vram(&self, data: &[u8; Vram::SIZE]) -> PPUResult<()> {
        match self.vram.try_borrow_mut() {
            Ok(mut vram) => {
                vram.overwrite(data);
                Ok(())
            }
            Err(_) => Err(PPUError::MemoryUnavailable {
                mem_name: String::from("vram"),
            }),
        }
    }

    pub fn overwrite_oam(&self, data: &[u8; Oam::SIZE]) -> PPUResult<()> {
        match self.oam.try_borrow_mut() {
            Ok(mut oam) => {
                oam.overwrite(data);
                Ok(())
            }
            Err(_) => Err(PPUError::MemoryUnavailable {
                mem_name: String::from("oam"),
            }),
        }
    }
}

impl FileOperation for PPUMem {
    fn read(&self, addr: Box<dyn Address>) -> Result<u8, Error> {
        match addr.area_type() {
            Area::Vram => match self.vram.try_borrow() {
                Ok(vram) => vram
                    .read(addr.get_address())
                    .ok_or_else(|| Error::SegmentationFault(addr.into())),
                Err(_) => Err(Error::SegmentationFault(addr.into())),
            },
            Area::Oam => match self.oam.try_borrow() {
                Ok(oam) => oam
                    .read(addr.get_address())
                    .ok_or_else(|| Error::SegmentationFault(addr.into())),
                Err(_) => Err(Error::SegmentationFault(addr.into())),
            },
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }

    fn write(&mut self, v: u8, addr: Box<dyn Address>) -> Result<(), Error> {
        match addr.area_type() {
            Area::Vram => match self.vram.try_borrow_mut() {
                Ok(mut vram) => vram
                    .write(addr.get_address(), v)
                    .ok_or_else(|| Error::SegmentationFault(addr.into())),
                Err(_) => Err(Error::SegmentationFault(addr.into())),
            },
            Area::Oam => match self.oam.try_borrow_mut() {
                Ok(mut oam) => oam
                    .write(addr.get_address(), v)
                    .ok_or_else(|| Error::SegmentationFault(addr.into())),
                Err(_) => Err(Error::SegmentationFault(addr.into())),
            },
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::PPU;

    #[test]
    fn blocked_vram_mut() {
        let ppu = PPU::new();
        let ppu_mem = ppu.memory();
        {
            let _lock = ppu.vram().borrow();
            assert!(ppu_mem.vram.try_borrow_mut().is_err());
        }
        {
            let _lock = ppu.vram().borrow_mut();
            assert!(ppu_mem.vram.try_borrow_mut().is_err());
        }
    }

    #[test]
    fn blocked_vram() {
        let ppu = PPU::new();
        let ppu_mem = ppu.memory();
        {
            let _lock = ppu.vram().borrow();
            assert!(ppu_mem.vram.try_borrow().is_ok());
        }
        {
            let _lock = ppu.vram().borrow_mut();
            assert!(ppu_mem.vram.try_borrow().is_err());
        }
    }

    #[test]
    fn blocked_oam_mut() {
        let ppu = PPU::new();
        let ppu_mem = ppu.memory();
        {
            let _lock = ppu.oam().borrow();
            assert!(ppu_mem.oam.try_borrow_mut().is_err());
        }
        {
            let _lock = ppu.oam().borrow_mut();
            assert!(ppu_mem.oam.try_borrow_mut().is_err());
        }
    }

    #[test]
    fn blocked_oam() {
        let ppu = PPU::new();
        let ppu_mem = ppu.memory();
        {
            let _lock = ppu.oam().borrow();
            assert!(ppu_mem.oam.try_borrow().is_ok());
        }
        {
            let _lock = ppu.oam().borrow_mut();
            assert!(ppu_mem.oam.try_borrow().is_err());
        }
    }
}
