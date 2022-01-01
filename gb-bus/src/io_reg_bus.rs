use crate::{address::Address, Address as PseudoAddress, Area, Error, FileOperation, IORegArea};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

pub struct IORegBus {
    areas: BTreeMap<IORegArea, Rc<RefCell<dyn FileOperation<IORegArea>>>>,
}

impl FileOperation<Area> for IORegBus {
    fn read(&self, address: Box<dyn PseudoAddress<Area>>) -> Result<u8, Error> {
        let addr: u16 = address.into();
        let reg = IORegArea::try_from(addr).map_err(|_e| Error::BusError(addr))?;

        if let Some(area) = self.areas.get(&reg) {
            #[cfg(feature = "trace_bus_rea")]
            log::trace!("reading at {:4x} in area {:?}", addr, reg);
            area.borrow().read(Box::new(Address::byte_reg(reg, addr)))
        } else {
            Err(Error::BusError(addr))
        }
    }

    fn write(&mut self, v: u8, address: Box<dyn PseudoAddress<Area>>) -> Result<(), Error> {
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
            area.borrow_mut()
                .write(v, Box::new(Address::byte_reg(reg, addr)))
        } else {
            Err(Error::BusError(addr))
        }
    }
}

#[derive(Default)]
pub struct IORegBusBuilder {
    areas: BTreeMap<IORegArea, Rc<RefCell<dyn FileOperation<IORegArea>>>>,
}

impl IORegBusBuilder {
    pub fn with_area(
        &mut self,
        area: IORegArea,
        handler: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    ) -> &mut Self {
        self.areas.insert(area, handler);
        self
    }

    pub fn build(self) -> IORegBus {
        IORegBus { areas: self.areas }
    }
}
