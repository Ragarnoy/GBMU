use super::Bios;
use gb_bus::{Addr, Address, Area, Error, FileOperation, IORegArea};
use std::{cell::RefCell, rc::Rc};

pub struct BiosWrapper {
    bios: Rc<RefCell<Bios>>,
    mbc: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
    pub bios_enabling_reg: u8,
}

impl BiosWrapper {
    pub fn new(
        bios: Rc<RefCell<Bios>>,
        mbc: Rc<RefCell<dyn FileOperation<Addr<Area>, Area>>>,
    ) -> Self {
        Self {
            bios,
            mbc,
            bios_enabling_reg: 0,
        }
    }

    fn bios_enabled(&self) -> bool {
        self.bios_enabling_reg == 0
    }

    fn read_bios(&self, address: Addr<Area>) -> Result<u8, Error> {
        self.bios.borrow().read(address)
    }

    fn read_mbc(&self, address: Addr<Area>) -> Result<u8, Error> {
        self.mbc.borrow().read(address)
    }

    fn write_bios(&self, v: u8, address: Addr<Area>) -> Result<(), Error> {
        self.bios.borrow_mut().write(v, address)
    }

    fn write_mbc(&self, v: u8, address: Addr<Area>) -> Result<(), Error> {
        self.mbc.borrow_mut().write(v, address)
    }
}

impl FileOperation<Addr<Area>, Area> for BiosWrapper {
    fn read(&self, address: Addr<Area>) -> Result<u8, Error> {
        let addr = address.get_address();
        if self.bios_enabled() && addr < self.bios.borrow().container.len() {
            self.read_bios(address)
        } else {
            self.read_mbc(address)
        }
    }

    fn write(&mut self, v: u8, address: Addr<Area>) -> Result<(), Error> {
        let addr = address.get_address();
        if self.bios_enabled() && addr < self.bios.borrow().container.len() {
            self.write_bios(v, address)
        } else {
            self.write_mbc(v, address)
        }
    }
}

impl FileOperation<Addr<IORegArea>, IORegArea> for BiosWrapper {
    fn read(&self, address: Addr<IORegArea>) -> Result<u8, Error> {
        let addr = address.get_address();
        if addr == 0 {
            Ok(self.bios_enabling_reg)
        } else {
            Err(Error::bus_error(address.into()))
        }
    }

    fn write(&mut self, v: u8, address: Addr<IORegArea>) -> Result<(), Error> {
        let addr = address.get_address();
        if addr == 0 {
            self.bios_enabling_reg = v;
            Ok(())
        } else {
            Err(Error::bus_error(address.into()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Area, BiosWrapper, FileOperation};

    #[test]
    fn overlap() {
        use crate::controllers::bios;
        use gb_bus::{address::Addr, generic::CharDevice};
        use std::{cell::RefCell, rc::Rc};

        let mbc_value = 42;

        let bios = bios::dmg();
        let bios = Rc::new(RefCell::new(bios));

        let mbc = Rc::new(RefCell::new(CharDevice(mbc_value)));

        let mut wrapper = BiosWrapper {
            bios,
            mbc: mbc.clone(),
            bios_enabling_reg: 0,
        };

        assert_eq!(
            wrapper.read(Addr::from_offset(Area::Rom, 0x42, 0)),
            Ok(234),
            "ensure we're able to read the bios"
        );

        assert_eq!(
            mbc.borrow().0,
            mbc_value,
            "ensure mbc is correctly initialised"
        );
        assert_eq!(
            wrapper.read(Addr::from_offset(Area::Rom, 0x1000, 0)),
            Ok(mbc_value),
            "ensure when we read outside of the bios size, we fallback to reading the rom"
        );

        let mbc_value = 69;
        assert_eq!(
            wrapper.write(mbc_value, Addr::from_offset(Area::Rom, 0x1000, 0)),
            Ok(()),
            "ensure when we write outside of the bios size, we fallback to writing to the rom"
        );
        assert_eq!(mbc.borrow().0, mbc_value);
    }
}
