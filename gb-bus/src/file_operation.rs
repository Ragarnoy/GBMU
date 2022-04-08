use crate::Error;

/// FileOperation basic trait to implement for a RAM Emulator or other area.
pub trait FileOperation<A, T>
where
    u16: From<A>,
    T: Into<u16>,
    A: Address<T>,
{
    fn write(&mut self, v: u8, addr: A, source: Option<Source>) -> Result<(), Error> {
        Err(Error::new_segfault(addr.into()))
    }

    fn read(&self, addr: A, source: Option<Source>) -> Result<u8, Error>;
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

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
pub enum Source {
    Cpu,
    Ppu,
    Dma,
    Debugger,
}

#[test]
fn test_comparing_lock_order() {
    assert!(Source::Cpu < Source::Ppu);
    assert!(Source::Ppu < Source::Dma);
    assert!(Source::Dma < Source::Debugger);
}
