use super::Bios;
use gb_bus::{Address, Area, Error, FileOperation, IORegArea, Source};
use std::{cell::RefCell, rc::Rc};

pub struct BiosWrapper<A>
where
    u16: From<A>,
    A: Address<Area>,
{
    bios: Rc<RefCell<Bios>>,
    mbc: Rc<RefCell<dyn FileOperation<A, Area>>>,
    pub bios_enabling_reg: u8,
}

impl<A> BiosWrapper<A>
where
    u16: From<A>,
    A: Address<Area>,
{
    pub fn new(bios: Rc<RefCell<Bios>>, mbc: Rc<RefCell<dyn FileOperation<A, Area>>>) -> Self {
        Self {
            bios,
            mbc,
            bios_enabling_reg: 0,
        }
    }

    fn bios_enabled(&self) -> bool {
        self.bios_enabling_reg == 0
    }

    fn read_bios(&self, addr: A, source: Option<Source>) -> Result<u8, Error> {
        self.bios.borrow().read(addr, source)
    }

    fn read_mbc(&self, addr: A, source: Option<Source>) -> Result<u8, Error> {
        self.mbc.borrow().read(addr, source)
    }

    fn write_bios(&self, v: u8, addr: A, source: Option<Source>) -> Result<(), Error> {
        self.bios.borrow_mut().write(v, addr, source)
    }

    fn write_mbc(&self, v: u8, addr: A, source: Option<Source>) -> Result<(), Error> {
        self.mbc.borrow_mut().write(v, addr, source)
    }
}

impl<A> FileOperation<A, Area> for BiosWrapper<A>
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, addr: A, source: Option<Source>) -> Result<u8, Error> {
        if self.bios_enabled() && addr.get_address() < self.bios.borrow().container.len() {
            self.read_bios(addr, source)
        } else {
            self.read_mbc(addr, source)
        }
    }

    fn write(&mut self, v: u8, addr: A, source: Option<Source>) -> Result<(), Error> {
        if self.bios_enabled() && addr.get_address() < self.bios.borrow().container.len() {
            self.write_bios(v, addr, source)
        } else {
            self.write_mbc(v, addr, source)
        }
    }
}

impl<A, B> FileOperation<B, IORegArea> for BiosWrapper<A>
where
    u16: From<A>,
    A: Address<Area>,
    u16: From<B>,
    B: Address<IORegArea>,
{
    fn read(&self, addr: B, _source: Option<Source>) -> Result<u8, Error> {
        if addr.get_address() == 0 {
            Ok(self.bios_enabling_reg)
        } else {
            Err(Error::bus_error(addr.into()))
        }
    }

    fn write(&mut self, v: u8, addr: B, _source: Option<Source>) -> Result<(), Error> {
        if addr.get_address() == 0 {
            self.bios_enabling_reg = v;
            Ok(())
        } else {
            Err(Error::bus_error(addr.into()))
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
            wrapper.read(Addr::from_offset(Area::Rom, 0x42, 0), None),
            Ok(234),
            "ensure we're able to read the bios"
        );

        assert_eq!(
            mbc.borrow().0,
            mbc_value,
            "ensure mbc is correctly initialised"
        );
        assert_eq!(
            wrapper.read(Addr::from_offset(Area::Rom, 0x1000, 0), None),
            Ok(mbc_value),
            "ensure when we read outside of the bios size, we fallback to reading the rom"
        );

        let mbc_value = 69;
        assert_eq!(
            wrapper.write(mbc_value, Addr::from_offset(Area::Rom, 0x1000, 0), None),
            Ok(()),
            "ensure when we write outside of the bios size, we fallback to writing to the rom"
        );
        assert_eq!(mbc.borrow().0, mbc_value);
    }
}
