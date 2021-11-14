use crate::{Address, Area, Error, FileOperation, IORegArea};

pub const RAM_BANK_SIZE: usize = 0x1000;
pub const CGB_MAX_BANKS: usize = 8;
pub const DMG_MAX_BANKS: usize = 2;

pub struct WorkingRam {
    banks: Vec<[u8; RAM_BANK_SIZE]>,
    selected_bank: u8,
    enable_cgb_feature: bool,
}

impl WorkingRam {
    pub fn new(enable_cgb_feature: bool) -> Self {
        let banks = if enable_cgb_feature {
            vec![[0; RAM_BANK_SIZE]; CGB_MAX_BANKS]
        } else {
            vec![[0; RAM_BANK_SIZE]; DMG_MAX_BANKS]
        };
        Self {
            banks,
            enable_cgb_feature,
            selected_bank: 1,
        }
    }
}

impl FileOperation<Area> for WorkingRam {
    fn write(&mut self, value: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        let address = addr.get_address();
        match address {
            0..=0xfff => self.banks[0][address] = value,
            0x1000..=0x1fff => {
                let address = address - 0x1000;
                let index = if self.enable_cgb_feature {
                    self.selected_bank as usize
                } else {
                    1
                };
                self.banks[index][address] = value;
            }
            _ => return Err(Error::new_bus_error(addr)),
        }
        Ok(())
    }

    fn read(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        let address = addr.get_address();
        match address {
            0..=0xfff => Ok(self.banks[0][address]),
            0x1000..=0x1fff => {
                let address = address - 0x1000;
                let index = if self.enable_cgb_feature {
                    self.selected_bank as usize
                } else {
                    1
                };
                Ok(self.banks[index][address])
            }
            _ => Err(Error::new_bus_error(addr)),
        }
    }
}

impl FileOperation<IORegArea> for WorkingRam {
    fn write(&mut self, value: u8, addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
        if self.enable_cgb_feature {
            self.selected_bank = value.clamp(1, 7);
            Ok(())
        } else {
            Err(Error::new_segfault(addr))
        }
    }

    fn read(&self, addr: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
        if self.enable_cgb_feature {
            Ok(self.selected_bank)
        } else {
            Err(Error::new_segfault(addr))
        }
    }
}
