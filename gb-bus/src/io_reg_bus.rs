use crate::{address::Address, Address as PseudoAddress, Area, Error, FileOperation, IORegArea};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

pub struct IORegBus {
    areas: BTreeMap<IORegArea, Rc<RefCell<dyn FileOperation<IORegArea>>>>,
}

impl FileOperation<Area> for IORegBus {
    fn read(&self, address: Box<dyn PseudoAddress<Area>>) -> Result<u8, Error> {
        let addr: u16 = address.into();
        let reg = IORegArea::from(addr);

        if let Some(area) = self.areas.get(&reg) {
            let offset = u16::from(reg) - addr;
            #[cfg(feature = "trace_bus_rea")]
            log::trace!("reading at {:4x} in area {:?}", addr, reg);
            area.borrow()
                .read(Box::new(Address::from_offset(reg, addr, offset)))
        } else {
            Err(Error::BusError(addr))
        }
    }

    fn write(&mut self, v: u8, address: Box<dyn PseudoAddress<Area>>) -> Result<(), Error> {
        let addr: u16 = address.into();
        let reg = IORegArea::from(addr);

        if let Some(area) = self.areas.get_mut(&reg) {
            let offset = u16::from(reg) - addr;
            #[cfg(feature = "trace_bus_write")]
            log::trace!(
                "writing at {:4x} the value {:2x} in area {:?}",
                addr,
                v,
                reg
            );
            area.borrow_mut()
                .write(v, Box::new(Address::from_offset(reg, addr, offset)))
        } else {
            Err(Error::BusError(addr))
        }
    }
}
