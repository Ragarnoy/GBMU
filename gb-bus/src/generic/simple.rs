use crate::{
    area::Area,
    error::Error,
    file_operation::{Address, FileOperation},
    io_reg_area::IORegArea,
};

pub struct SimpleRW<const SIZE: usize> {
    store: [u8; SIZE],
}

impl<const SIZE: usize> FileOperation<Area> for SimpleRW<SIZE> {
    fn write(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
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

    fn read(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        let address = addr.get_address();
        #[cfg(feature = "trace_simple_rw_read")]
        log::trace!("reading value abs={:x}, rel={:x}", u16::from(addr), address);
        Ok(self.store[address])
    }
}

impl<const SIZE: usize> FileOperation<IORegArea> for SimpleRW<SIZE> {
    fn write(&mut self, v: u8, addr: Box<dyn Address<IORegArea>>) -> Result<(), Error> {
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

    fn read(&self, addr: Box<dyn Address<IORegArea>>) -> Result<u8, Error> {
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
