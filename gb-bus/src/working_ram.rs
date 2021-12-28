use crate::{generic::DynBankableStorage, Addr, Address, Area, Error, FileOperation, IORegArea};

pub const RAM_BANK_SIZE: usize = 0x1000;
pub const CGB_MAX_BANKS: usize = 8;
pub const DMG_MAX_BANKS: usize = 2;

pub struct WorkingRam {
    storage: DynBankableStorage<RAM_BANK_SIZE>,
    enable_cgb_feature: bool,
}

impl WorkingRam {
    pub fn new(enable_cgb_feature: bool) -> Self {
        let mut storage = if enable_cgb_feature {
            DynBankableStorage::with_bank_amount(CGB_MAX_BANKS)
        } else {
            DynBankableStorage::with_bank_amount(DMG_MAX_BANKS)
        };
        storage.set_bank_index(1);
        Self {
            storage,
            enable_cgb_feature,
        }
    }
}

impl FileOperation<Addr<Area>, Area> for WorkingRam {
    fn write(&mut self, value: u8, addr: Addr<Area>) -> Result<(), Error> {
        let address = addr.get_address();
        match address {
            0..=0xfff => self.storage.root_bank_mut()[address] = value,
            0x1000..=0x1fff => {
                let address = address - 0x1000;
                self.storage[address] = value;
            }
            _ => return Err(Error::bus_error(addr.into())),
        }
        Ok(())
    }

    fn read(&self, addr: Addr<Area>) -> Result<u8, Error> {
        let address = addr.get_address();
        match address {
            0..=0xfff => Ok(self.storage.root_bank()[address]),
            0x1000..=0x1fff => {
                let address = address - 0x1000;
                Ok(self.storage[address])
            }
            _ => Err(Error::bus_error(addr.into())),
        }
    }
}

impl FileOperation<Addr<IORegArea>, IORegArea> for WorkingRam {
    fn write(&mut self, value: u8, addr: Addr<IORegArea>) -> Result<(), Error> {
        if self.enable_cgb_feature {
            self.storage.set_bank_index((value & 0x7).min(1) as usize);
            Ok(())
        } else {
            Err(Error::new_segfault(addr.into()))
        }
    }

    fn read(&self, addr: Addr<IORegArea>) -> Result<u8, Error> {
        if self.enable_cgb_feature {
            Ok(self.storage.current_bank_index as u8)
        } else {
            Err(Error::new_segfault(addr.into()))
        }
    }
}
