use super::{Lockable, Oam, Vram};
use crate::error::{PPUError, PPUResult};
use crate::UNDEFINED_VALUE;
use gb_bus::{Address, Area, Error, FileOperation};
use std::cell::RefCell;
use std::rc::Rc;

/// Allow external structures to read/write the memory of the ppu.
///
/// The read/write operation might be ignored if the ppu is currently using the concerned memory area.
pub struct PPUMem {
    vram: Rc<RefCell<Vram>>,
    oam: Rc<RefCell<Oam>>,
}

impl PPUMem {
    /// Build a PPUMem from references counters of Vram and Oam.
    ///
    /// This function is used by [PPU.memory()](crate::PPU::memory), you should not need to call this constructor yourself.
    pub fn new(vram: Rc<RefCell<Vram>>, oam: Rc<RefCell<Oam>>) -> Self {
        PPUMem { vram, oam }
    }

    /// Completely replace the vram of the ppu,if it is not currently using it.
    ///
    /// This function exist for debugging purpose.
    pub fn overwrite_vram(&self, data: &[u8; Vram::SIZE]) -> PPUResult<()> {
        match self.vram.try_borrow_mut() {
            Ok(mut vram) => {
                vram.overwrite(data);
                log::info!("overwritting vram");
                Ok(())
            }
            Err(_) => Err(PPUError::MemoryUnavailable {
                mem_name: String::from("vram"),
            }),
        }
    }

    /// Completely replace the oam of the ppu,if it is not currently using it.
    ///
    /// This function exist for debugging purpose.
    pub fn overwrite_oam(&self, data: &[u8; Oam::SIZE]) -> PPUResult<()> {
        match self.oam.try_borrow_mut() {
            Ok(mut oam) => {
                oam.overwrite(data);
                log::info!("overwritting oam");
                Ok(())
            }
            Err(_) => Err(PPUError::MemoryUnavailable {
                mem_name: String::from("oam"),
            }),
        }
    }
}

impl FileOperation<Area> for PPUMem {
    /// Read a value from memory. If the concerned memory area is currently locked an undefined value is returned.
    fn read(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        match addr.area_type() {
            Area::Vram => match self.vram.try_borrow_mut() {
                Ok(vram) => {
                    if vram.get_lock().is_none() {
                        vram.read(addr.get_address())
                            .map_err(|_| Error::SegmentationFault(addr.into()))
                    } else {
                        Ok(UNDEFINED_VALUE)
                    }
                }
                Err(err) => {
                    log::error!("failed vram read: {}", err);
                    Ok(UNDEFINED_VALUE)
                }
            },
            Area::Oam => match self.oam.try_borrow() {
                Ok(oam) => {
                    if oam.get_lock().is_none() {
                        oam.read(addr.get_address())
                            .map_err(|_| Error::SegmentationFault(addr.into()))
                    } else {
                        Ok(UNDEFINED_VALUE)
                    }
                }
                Err(err) => {
                    log::error!("failed oam read: {}", err);
                    Ok(UNDEFINED_VALUE)
                }
            },
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }

    /// Write value into memory. If the concerned memory area is currently locked, nothing is done.
    fn write(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        match addr.area_type() {
            Area::Vram => match self.vram.try_borrow_mut() {
                Ok(mut vram) => {
                    if vram.get_lock().is_none() {
                        vram.write(addr.get_address(), v)
                            .map_err(|_| Error::SegmentationFault(addr.into()))
                    } else {
                        Ok(())
                    }
                }
                Err(err) => {
                    log::error!("failed vram write: {}", err);
                    Ok(())
                }
            },
            Area::Oam => match self.oam.try_borrow_mut() {
                Ok(mut oam) => {
                    if oam.get_lock().is_none() {
                        oam.write(addr.get_address(), v)
                            .map_err(|_| Error::SegmentationFault(addr.into()))
                    } else {
                        Ok(())
                    }
                }
                Err(err) => {
                    log::error!("failed oam write: {}", err);
                    Ok(())
                }
            },
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
}

#[cfg(test)]
mod read {
    use super::PPUMem;
    use crate::memory::{Oam, Vram};
    use crate::test_tools::TestAddress;
    use gb_bus::FileOperation;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn vram() {
        let vram = Rc::new(RefCell::new([0x42; Vram::SIZE].into()));
        let oam = Rc::new(RefCell::new(Oam::default()));
        let ppu_mem = PPUMem::new(vram, oam);

        let res = ppu_mem
            .read(Box::new(TestAddress::root_vram()))
            .expect("Try reading value from vram");
        assert_eq!(res, 0x42, "invalid value from vram");
    }

    #[test]
    fn oam() {
        let vram = Rc::new(RefCell::new(Vram::default()));
        let oam = Rc::new(RefCell::new([0x42; Oam::SIZE].into()));
        let ppu_mem = PPUMem::new(vram, oam);

        let res = ppu_mem
            .read(Box::new(TestAddress::root_oam()))
            .expect("Try reading value from vram");
        assert_eq!(res, 0x42, "invalid value from vram");
    }
}

#[cfg(test)]
mod write {
    use super::PPUMem;
    use crate::memory::{Oam, Vram};
    use crate::test_tools::TestAddress;
    use gb_bus::FileOperation;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn vram() {
        let vram = Rc::new(RefCell::new(Vram::default()));
        let oam = Rc::new(RefCell::new(Oam::default()));
        let mut ppu_mem = PPUMem::new(vram, oam);

        ppu_mem
            .write(0x42, Box::new(TestAddress::root_vram()))
            .expect("Try write value into vram");
        let res = ppu_mem
            .read(Box::new(TestAddress::root_vram()))
            .expect("Try reading value from vram");
        assert_eq!(res, 0x42, "invalid value from vram");
    }

    #[test]
    fn oam() {
        let vram = Rc::new(RefCell::new(Vram::default()));
        let oam = Rc::new(RefCell::new(Oam::default()));
        let mut ppu_mem = PPUMem::new(vram, oam);

        ppu_mem
            .write(0x42, Box::new(TestAddress::root_oam()))
            .expect("Try write value into oam");
        let res = ppu_mem
            .read(Box::new(TestAddress::root_oam()))
            .expect("Try reading value from oam");
        assert_eq!(res, 0x42, "invalid value from oam");
    }
}
