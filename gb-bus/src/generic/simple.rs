use crate::Source;
use crate::{area::Area, error::Error, io_reg_area::IORegArea, Address, FileOperation};

pub struct SimpleRW<const SIZE: usize> {
    store: [u8; SIZE],
}

#[cfg(feature = "serialization")]
impl<const SIZE: usize> SimpleRW<SIZE> {
    pub fn save(&self) -> Vec<u8> {
        self.store.to_vec()
    }
}

#[cfg(feature = "serialization")]
impl<const SIZE: usize> TryFrom<Vec<u8>> for SimpleRW<SIZE> {
    type Error = usize;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        <[u8; SIZE]>::try_from(value)
            .map_err(|v| v.len())
            .map(|store| Self { store })
    }
}

impl<A, const SIZE: usize> FileOperation<A, Area> for SimpleRW<SIZE>
where
    u16: From<A>,
    A: Address<Area>,
{
    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        let address = addr.get_address();
        #[cfg(feature = "trace_simple_rw_write")]
        log::trace!(
            "writing value v={:x}, abs={:x}, rel={:x}",
            v,
            u16::from(addr),
            address
        );
        self.store[address] = v;
        Ok(())
    }

    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        let address = addr.get_address();
        #[cfg(feature = "trace_simple_rw_read")]
        log::trace!("reading value abs={:x}, rel={:x}", u16::from(addr), address);
        Ok(self.store[address])
    }
}

impl<A, const SIZE: usize> FileOperation<A, IORegArea> for SimpleRW<SIZE>
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn write(&mut self, v: u8, addr: A, _source: Option<Source>) -> Result<(), Error> {
        let address = addr.get_address();
        #[cfg(feature = "trace_simple_rw_write")]
        log::trace!(
            "writing value v={:x}, abs={:x}, rel={:x}",
            v,
            u16::from(addr),
            address
        );
        self.store[address] = v;
        Ok(())
    }

    fn read(&self, addr: A, _source: Option<Source>) -> Result<u8, Error> {
        let address = addr.get_address();
        #[cfg(feature = "trace_simple_rw_read")]
        log::trace!("reading value abs={:x}, rel={:x}", u16::from(addr), address);
        Ok(self.store[address])
    }
}

impl<const SIZE: usize> Default for SimpleRW<SIZE> {
    fn default() -> Self {
        Self { store: [0; SIZE] }
    }
}
