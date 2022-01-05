use crate::{Addr, Address, Area, Error, FileOperation, IORegArea};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

type IORegElement = dyn FileOperation<Addr<IORegArea>, IORegArea>;

pub struct IORegBus {
    areas: BTreeMap<IORegArea, Rc<RefCell<IORegElement>>>,
}

impl<A> FileOperation<A, Area> for IORegBus
where
    u16: From<A>,
    A: Address<Area> + std::fmt::Debug,
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
        let reg = IORegArea::try_from(addr).map_err(|_e| Error::InvalidIORegAddress(addr))?;

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

macro_rules! new_chardev {
    () => {
        std::rc::Rc::new(std::cell::RefCell::new(crate::generic::CharDevice(0)))
    };
}

#[derive(Default)]
pub struct IORegBusBuilder {
    areas: BTreeMap<IORegArea, Rc<RefCell<IORegElement>>>,
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

    pub fn with_default_sound(&mut self) -> &mut Self {
        self.with_area(IORegArea::Nr10, new_chardev!())
            .with_area(IORegArea::Nr11, new_chardev!())
            .with_area(IORegArea::Nr12, new_chardev!())
            .with_area(IORegArea::Nr13, new_chardev!())
            .with_area(IORegArea::Nr14, new_chardev!())
            .with_area(IORegArea::Nr21, new_chardev!())
            .with_area(IORegArea::Nr22, new_chardev!())
            .with_area(IORegArea::Nr23, new_chardev!())
            .with_area(IORegArea::Nr24, new_chardev!())
            .with_area(IORegArea::Nr30, new_chardev!())
            .with_area(IORegArea::Nr31, new_chardev!())
            .with_area(IORegArea::Nr32, new_chardev!())
            .with_area(IORegArea::Nr33, new_chardev!())
            .with_area(IORegArea::Nr34, new_chardev!())
            .with_area(IORegArea::Nr41, new_chardev!())
            .with_area(IORegArea::Nr42, new_chardev!())
            .with_area(IORegArea::Nr43, new_chardev!())
            .with_area(IORegArea::Nr44, new_chardev!())
            .with_area(IORegArea::Nr50, new_chardev!())
            .with_area(IORegArea::Nr51, new_chardev!())
            .with_area(IORegArea::Nr52, new_chardev!())
    }

    pub fn with_default_waveform_ram(&mut self) -> &mut Self {
        self.with_area(IORegArea::WaveRam0, new_chardev!())
            .with_area(IORegArea::WaveRam1, new_chardev!())
            .with_area(IORegArea::WaveRam2, new_chardev!())
            .with_area(IORegArea::WaveRam3, new_chardev!())
            .with_area(IORegArea::WaveRam4, new_chardev!())
            .with_area(IORegArea::WaveRam5, new_chardev!())
            .with_area(IORegArea::WaveRam6, new_chardev!())
            .with_area(IORegArea::WaveRam7, new_chardev!())
            .with_area(IORegArea::WaveRam8, new_chardev!())
            .with_area(IORegArea::WaveRam9, new_chardev!())
            .with_area(IORegArea::WaveRamA, new_chardev!())
            .with_area(IORegArea::WaveRamB, new_chardev!())
            .with_area(IORegArea::WaveRamC, new_chardev!())
            .with_area(IORegArea::WaveRamD, new_chardev!())
            .with_area(IORegArea::WaveRamE, new_chardev!())
            .with_area(IORegArea::WaveRamF, new_chardev!())
    }

    pub fn with_default_serial(&mut self) -> &mut Self {
        self.with_area(IORegArea::SB, new_chardev!())
            .with_area(IORegArea::SC, new_chardev!())
    }

    pub fn build(self) -> IORegBus {
        IORegBus { areas: self.areas }
    }
}
