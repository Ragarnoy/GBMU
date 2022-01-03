use crate::{Addr, Address, Area, Error, FileOperation, IORegArea};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

pub struct IORegBus {
    areas: BTreeMap<IORegArea, Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>>,
}

impl<A> FileOperation<A, Area> for IORegBus
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, address: A) -> Result<u8, Error> {
        let addr: u16 = address.into();
        let reg = IORegArea::try_from(addr).map_err(|_e| Error::BusError(addr))?;

        if let Some(area) = self.areas.get(&reg) {
            #[cfg(feature = "trace_bus_read")]
            log::trace!("reading at {:4x} in area {:?}", addr, reg);
            area.borrow().read(Addr::byte_reg(reg, addr))
        } else {
            Err(Error::BusError(addr))
        }
    }

    fn write(&mut self, v: u8, address: A) -> Result<(), Error> {
        let addr: u16 = address.into();
        let reg = IORegArea::try_from(addr).map_err(|_e| Error::BusError(addr))?;

        if let Some(area) = self.areas.get_mut(&reg) {
            #[cfg(feature = "trace_bus_write")]
            log::trace!(
                "writing at {:4x} the value {:2x} in area {:?}",
                addr,
                v,
                reg
            );
            area.borrow_mut().write(v, Addr::byte_reg(reg, addr))
        } else {
            Err(Error::BusError(addr))
        }
    }
}

#[derive(Default)]
pub struct IORegBusBuilder {
    areas: BTreeMap<IORegArea, Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>>,
}

impl IORegBusBuilder {
    pub fn with_area(
        &mut self,
        area: IORegArea,
        handler: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    ) -> &mut Self {
        self.areas.insert(area, handler);
        self
    }

    pub fn build(self) -> IORegBus {
        IORegBus { areas: self.areas }
    }
}
