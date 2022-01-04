use crate::{Addr, Address, Area, Error, FileOperation, IORegArea};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

type IORegElement = dyn FileOperation<Addr<IORegArea>, IORegArea>;

pub struct IORegBus {
    areas: BTreeMap<IORegArea, Rc<RefCell<IORegElement>>>,
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
        use crate::generic::CharDevice;

        self.with_area(IORegArea::Nr10, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr11, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr12, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr13, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr14, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr21, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr22, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr23, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr24, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr30, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr31, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr32, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr33, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr34, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr41, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr42, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr43, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr44, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr50, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr51, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::Nr52, Rc::new(RefCell::new(CharDevice(0))))
    }

    pub fn with_default_waveform_ram(&mut self) -> &mut Self {
        use crate::generic::CharDevice;

        self.with_area(IORegArea::WaveRam0, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRam1, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRam2, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRam3, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRam4, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRam5, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRam6, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRam7, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRam8, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRam9, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRamA, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRamB, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRamC, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRamD, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRamE, Rc::new(RefCell::new(CharDevice(0))))
            .with_area(IORegArea::WaveRamF, Rc::new(RefCell::new(CharDevice(0))))
    }

    pub fn build(self) -> IORegBus {
        IORegBus { areas: self.areas }
    }
}
