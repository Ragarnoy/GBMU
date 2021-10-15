pub mod array {
    use gb_bus::{Bus, Error};

    #[derive(Clone, Debug)]
    pub struct Mock {
        store: [u8; u16::MAX as usize],
    }

    impl Default for Mock {
        fn default() -> Self {
            Self {
                store: [0; u16::MAX as usize],
            }
        }
    }

    impl Bus<u8> for Mock {
        fn read(&self, address: u16) -> Result<u8, Error> {
            Ok(self.store[address as usize])
        }

        fn write(&mut self, address: u16, data: u8) -> Result<(), Error> {
            self.store[address as usize] = data;
            Ok(())
        }
    }

    impl Bus<u16> for Mock {
        fn read(&self, _address: u16) -> Result<u16, Error> {
            unimplemented!();
        }

        fn write(&mut self, _address: u16, _data: u16) -> Result<(), Error> {
            unimplemented!();
        }
    }
}

pub mod binary {
    use gb_bus::{Bus, Error};
    use std::collections::BTreeMap;

    #[derive(Clone, Debug)]
    pub struct Mock {
        store: BTreeMap<u16, u8>,
    }

    impl Default for Mock {
        fn default() -> Self {
            Self {
                store: BTreeMap::new(),
            }
        }
    }

    impl Bus<u8> for Mock {
        fn read(&self, address: u16) -> Result<u8, Error> {
            let res = self
                .store
                .get(&address)
                .map(|v| *v)
                .ok_or_else(|| Error::SegmentationFault(address));

            res
        }

        fn write(&mut self, address: u16, value: u8) -> Result<(), Error> {
            self.store.insert(address, value);
            Ok(())
        }
    }

    impl Bus<u16> for Mock {
        fn read(&self, _address: u16) -> Result<u16, Error> {
            unimplemented!();
        }

        fn write(&mut self, _address: u16, _data: u16) -> Result<(), Error> {
            unimplemented!();
        }
    }
}

pub use binary::Mock;
