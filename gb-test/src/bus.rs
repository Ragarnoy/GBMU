pub mod array {
    use gb_bus::{Area, Bus, Error, Lock, MemoryLock};

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
        fn read(&self, address: u16, _lock_key: Option<Lock>) -> Result<u8, Error> {
            Ok(self.store[address as usize])
        }

        fn write(&mut self, address: u16, data: u8, _lock_key: Option<Lock>) -> Result<(), Error> {
            self.store[address as usize] = data;
            Ok(())
        }
    }

    impl Bus<u16> for Mock {
        fn read(&self, _address: u16, _lock_key: Option<Lock>) -> Result<u16, Error> {
            unimplemented!();
        }

        fn write(
            &mut self,
            _address: u16,
            _data: u16,
            _lock_key: Option<Lock>,
        ) -> Result<(), Error> {
            unimplemented!();
        }
    }

    impl MemoryLock for Mock {
        fn lock(&mut self, _area: Area, _lock: Lock) {}

        fn unlock(&mut self, _area: Area) {}

        fn is_available(&self, _address: u16, _lock_key: Option<Lock>) -> bool {
            true
        }
    }
}

pub mod binary {
    use gb_bus::{Area, Bus, Error, Lock, MemoryLock};
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
        fn read(&self, address: u16, _lock_key: Option<Lock>) -> Result<u8, Error> {
            let res = self
                .store
                .get(&address)
                .copied()
                .ok_or(Error::SegmentationFault(address));

            res
        }

        fn write(&mut self, address: u16, value: u8, _lock_key: Option<Lock>) -> Result<(), Error> {
            self.store.insert(address, value);
            Ok(())
        }
    }

    impl Bus<u16> for Mock {
        fn read(&self, _address: u16, _lock_key: Option<Lock>) -> Result<u16, Error> {
            unimplemented!();
        }

        fn write(
            &mut self,
            _address: u16,
            _data: u16,
            _lock_key: Option<Lock>,
        ) -> Result<(), Error> {
            unimplemented!();
        }
    }

    impl MemoryLock for Mock {
        fn lock(&mut self, _area: Area, _lock: Lock) {}

        fn unlock(&mut self, _area: Area) {}

        fn is_available(&self, _address: u16, _lock_key: Option<Lock>) -> bool {
            true
        }
    }
}

pub use binary::Mock;
