use crate::{Area, Error};

/// FileOperation basic trait to implement for a RAM Emulator or other area.
pub trait FileOperation<A, T>
where
    u16: From<A>,
    T: Into<u16>,
    A: Address<T>,
{
    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        let _v = v;
        Err(Error::new_segfault(addr.into()))
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

#[cfg_attr(
    feature = "serialization",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
pub enum Lock {
    Ppu,
    Dma,
    Debugger,
}

pub trait MemoryLock {
    /// Lock a memory area for exclusive access
    fn lock(&mut self, area: Area, lock: Lock);
    /// unlock a memory area to restore universal access
    fn unlock(&mut self, area: Area);
    /// Detect if area is locked from an address
    fn is_available(&self, area: Area, lock_key: Option<Lock>) -> bool;
}

pub trait InternalLock<A, T>: MemoryLock + FileOperation<A, T>
where
    u16: From<A>,
    A: Address<T>,
    T: Into<u16>,
{
}

#[test]
fn test_comparing_lock_order() {
    assert!(Lock::Ppu < Lock::Dma);
    assert!(Lock::Dma < Lock::Debugger);
    assert!(Lock::Ppu < Lock::Debugger);
}
