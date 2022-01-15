use crate::{Address, Area, Error, FileOperation, IORegArea};

pub const RAM_BANK_SIZE: usize = 0x1000;
pub const CGB_MAX_BANKS: usize = 8;
pub const DMG_MAX_BANKS: usize = 2;

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Debug, Clone)]
pub struct WorkingRam {
    storage: Vec<u8>,
    bank: u8,
    enable_cgb_feature: bool,
}

impl WorkingRam {
    pub fn new(enable_cgb_feature: bool) -> Self {
        let storage = if enable_cgb_feature {
            Vec::with_capacity(CGB_MAX_BANKS * RAM_BANK_SIZE)
        } else {
            Vec::with_capacity(DMG_MAX_BANKS * RAM_BANK_SIZE)
        };
        Self {
            storage,
            bank: 1,
            enable_cgb_feature,
        }
    }

    fn offset_addr(&self, addr: usize) -> usize {
        let bank = self.get_bank_for_addr(addr);
        bank * RAM_BANK_SIZE | (addr & 0xfff) as usize
    }

    fn get_bank_for_addr(&self, addr: usize) -> usize {
        if addr <= 0xfff {
            0
        } else {
            self.bank as usize
        }
    }
}

impl<A> FileOperation<A, Area> for WorkingRam
where
    u16: From<A>,
    A: Address<Area>,
{
    fn write(&mut self, value: u8, addr: A) -> Result<(), Error> {
        let address = addr.get_address();
        let offset_addr = self.offset_addr(address);
        self.storage[offset_addr] = value;
        Ok(())
    }

    fn read(&self, addr: A) -> Result<u8, Error> {
        let address = addr.get_address();
        let offset_addr = self.offset_addr(address);
        Ok(self.storage[offset_addr])
    }
}

impl<A> FileOperation<A, IORegArea> for WorkingRam
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn write(&mut self, value: u8, addr: A) -> Result<(), Error> {
        if self.enable_cgb_feature {
            self.bank = (value & 0x7).min(1);
            Ok(())
        } else {
            Err(Error::new_segfault(addr.into()))
        }
    }

    fn read(&self, addr: A) -> Result<u8, Error> {
        if self.enable_cgb_feature {
            Ok(self.bank)
        } else {
            Err(Error::new_segfault(addr.into()))
        }
    }
}
