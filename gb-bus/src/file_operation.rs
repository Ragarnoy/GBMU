use crate::Error;

/// FileOperation basic trait to implement for a RAM Emulator or other area.
pub trait FileOperation<A, T>
where
    T: Into<u16>,
    A: Address<T>,
{
    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        let _v = v;
        // Err(Error::new_segfault(addr))
        Ok(())
    }

    fn read(&self, addr: A) -> Result<u8, Error>;
}

pub trait Address<A> {
    /// Return the relative address in the current area
    fn get_address(&self) -> usize;

    /// Return the current area type
    fn area_type(&self) -> A;
}

impl<A: PartialEq + Eq> PartialEq for dyn Address<A> {
    fn eq(&self, other: &Self) -> bool {
        self.get_address() == other.get_address() && self.area_type() == other.area_type()
    }
}
