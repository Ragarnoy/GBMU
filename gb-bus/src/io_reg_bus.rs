use crate::{Addr, Address, Area, Error, FileOperation, IORegArea, Source};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

macro_rules! new_chardev {
    () => {
        std::rc::Rc::new(std::cell::RefCell::new(crate::generic::CharDevice(0)))
    };
}

type IORegElement = dyn FileOperation<Addr<IORegArea>, IORegArea>;
type IORegNode = Rc<RefCell<IORegElement>>;

#[derive(Default)]
pub struct IORegBus {
    areas: BTreeMap<IORegArea, IORegNode>,
}

impl<A> FileOperation<A, Area> for IORegBus
where
    u16: From<A>,
    A: Address<Area> + std::fmt::Debug,
{
    fn read(&self, address: A, source: Option<Source>) -> Result<u8, Error> {
        let addr: u16 = address.into();
        let reg = IORegArea::try_from(addr).map_err(|_e| Error::BusError(addr))?;

        if let Some(area) = self.areas.get(&reg) {
            #[cfg(feature = "trace_bus_read")]
            log::trace!("reading at {:4x} in area {:?} from {:?}", addr, reg, source);
            area.borrow().read(Addr::byte_reg(reg, addr), source)
        } else {
            Err(Error::BusError(addr))
        }
    }

    fn write(&mut self, v: u8, address: A, source: Option<Source>) -> Result<(), Error> {
        let addr: u16 = address.into();
        let reg = IORegArea::try_from(addr).map_err(|_e| Error::InvalidIORegAddress(addr))?;

        if let Some(area) = self.areas.get_mut(&reg) {
            #[cfg(feature = "trace_bus_write")]
            log::trace!(
                "writing at {:4x} the value {:2x} in area {:?} from {:?}",
                addr,
                v,
                reg,
                source
            );
            area.borrow_mut()
                .write(v, Addr::byte_reg(reg, addr), source)
        } else {
            Err(Error::BusError(addr))
        }
    }
}

impl IORegBus {
    pub fn with_area(&mut self, area: IORegArea, handler: IORegNode) -> &mut Self {
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

    pub fn with_serial(&mut self, serial: IORegNode) -> &mut Self {
        use IORegArea::{SB, SC};

        self.with_area(SB, serial.clone()).with_area(SC, serial)
    }

    pub fn with_timer(&mut self, timer: IORegNode) -> &mut Self {
        use IORegArea::{Div, Tac, Tima, Tma};

        self.with_area(Div, timer.clone())
            .with_area(Tima, timer.clone())
            .with_area(Tac, timer.clone())
            .with_area(Tma, timer)
    }

    pub fn with_ppu(&mut self, ppu: IORegNode) -> &mut Self {
        use IORegArea::{Bgp, LcdControl, LcdStat, Ly, Lyc, Obp0, Obp1, Scx, Scy, Wx, Wy};

        self.with_area(LcdControl, ppu.clone())
            .with_area(LcdStat, ppu.clone())
            .with_area(Scy, ppu.clone())
            .with_area(Scx, ppu.clone())
            .with_area(Ly, ppu.clone())
            .with_area(Lyc, ppu.clone())
            .with_area(Bgp, ppu.clone())
            .with_area(Obp0, ppu.clone())
            .with_area(Obp1, ppu.clone())
            .with_area(Wy, ppu.clone())
            .with_area(Wx, ppu)
    }

    pub fn with_ppu_cgb(&mut self, ppu: IORegNode) -> &mut Self {
        use IORegArea::{Bcpd, Bcps, Ocpd, Ocps, Opri, Vbk};

        self.with_area(Vbk, ppu.clone())
            .with_area(Opri, ppu.clone())
            .with_area(Bcps, ppu.clone())
            .with_area(Bcpd, ppu.clone())
            .with_area(Ocps, ppu.clone())
            .with_area(Ocpd, ppu.clone())
    }

    pub fn with_hdma(&mut self, hdma: IORegNode) -> &mut Self {
        self.with_area(IORegArea::Hdma1, hdma.clone())
            .with_area(IORegArea::Hdma2, hdma.clone())
            .with_area(IORegArea::Hdma3, hdma.clone())
            .with_area(IORegArea::Hdma4, hdma.clone())
            .with_area(IORegArea::Hdma5, hdma)
    }
}
