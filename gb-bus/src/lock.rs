use crate::{file_operation::Address, Area, FileOperation};

#[derive(
    Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord, serde::Deserialize, serde::Serialize,
)]
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
