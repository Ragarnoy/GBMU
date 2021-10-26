use gb_bus::{Address, Area, Error, FileOperation, IORegArea};
use std::{cell::RefCell, rc::Rc};

pub struct BiosWrapper {
    bios: Rc<RefCell<dyn FileOperation<Area>>>,
    mbc: Rc<RefCell<dyn FileOperation<Area>>>,
    bios_enabling_reg: u8,
}

impl BiosWrapper {
    pub fn new(
        bios: Rc<RefCell<dyn FileOperation<Area>>>,
        mbc: Rc<RefCell<dyn FileOperation<Area>>>,
    ) -> Self {
        Self {
            bios,
            mbc,
            bios_enabling_reg: 0,
        }
    }

    fn bios_enabled(&self) -> bool {
        self.bios_enabling_reg != 0
    }
}

impl FileOperation<Area> for BiosWrapper {
    fn read(&self, address: Box<dyn Address<Area>>) -> Result<u8, Error> {
        if self.bios_enabled() {
            self.bios.borrow().read(address)
        } else {
            self.mbc.borrow().read(address)
        }
    }

    fn write(&mut self, v: u8, address: Box<dyn Address<Area>>) -> Result<(), Error> {
        if self.bios_enabled() {
            self.bios.borrow_mut().write(v, address)
        } else {
            self.mbc.borrow_mut().write(v, address)
        }
    }
}

impl FileOperation<IORegArea> for BiosWrapper {
    fn read(&self, address: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
        let addr: u16 = address.into();
        if addr == 0 {
            Ok(self.bios_enabling_reg)
        } else {
            Err(Error::BusError(addr))
        }
    }

    fn write(&mut self, v: u8, address: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        let addr: u16 = address.into();
        if addr == 0 {
            self.bios_enabling_reg = v;
            Ok(())
        } else {
            Err(Error::BusError(addr))
        }
    }
}
