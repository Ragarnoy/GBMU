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

    pub fn with_sound(&mut self, apu: IORegNode) -> &mut Self {
        use IORegArea::{
            Nr10, Nr11, Nr12, Nr13, Nr14, Nr21, Nr22, Nr23, Nr24, Nr30, Nr31, Nr32, Nr33, Nr34,
            Nr41, Nr42, Nr43, Nr44, Nr50, Nr51, Nr52, WaveRam0, WaveRam1, WaveRam2, WaveRam3,
            WaveRam4, WaveRam5, WaveRam6, WaveRam7, WaveRam8, WaveRam9, WaveRamA, WaveRamB,
            WaveRamC, WaveRamD, WaveRamE, WaveRamF,
        };
        self.with_area(Nr10, apu.clone())
            .with_area(Nr11, apu.clone())
            .with_area(Nr12, apu.clone())
            .with_area(Nr13, apu.clone())
            .with_area(Nr14, apu.clone())
            .with_area(Nr21, apu.clone())
            .with_area(Nr22, apu.clone())
            .with_area(Nr23, apu.clone())
            .with_area(Nr24, apu.clone())
            .with_area(Nr30, apu.clone())
            .with_area(Nr31, apu.clone())
            .with_area(Nr32, apu.clone())
            .with_area(Nr33, apu.clone())
            .with_area(Nr34, apu.clone())
            .with_area(Nr41, apu.clone())
            .with_area(Nr42, apu.clone())
            .with_area(Nr43, apu.clone())
            .with_area(Nr44, apu.clone())
            .with_area(Nr50, apu.clone())
            .with_area(Nr51, apu.clone())
            .with_area(Nr52, apu.clone())
            .with_area(WaveRam0, apu.clone())
            .with_area(WaveRam1, apu.clone())
            .with_area(WaveRam2, apu.clone())
            .with_area(WaveRam3, apu.clone())
            .with_area(WaveRam4, apu.clone())
            .with_area(WaveRam5, apu.clone())
            .with_area(WaveRam6, apu.clone())
            .with_area(WaveRam7, apu.clone())
            .with_area(WaveRam8, apu.clone())
            .with_area(WaveRam9, apu.clone())
            .with_area(WaveRamA, apu.clone())
            .with_area(WaveRamB, apu.clone())
            .with_area(WaveRamC, apu.clone())
            .with_area(WaveRamD, apu.clone())
            .with_area(WaveRamE, apu.clone())
            .with_area(WaveRamF, apu.clone())
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

    #[cfg(feature = "cgb")]
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
