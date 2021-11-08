use crate::{Area, FileOperation};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Lock {
    Ppu,
    Debugger,
    Dma,
}

pub trait MemoryLock {
    /// Lock a memory area for exclusive access
    fn lock(&mut self, area: Area, lock: Lock);
    /// unlock a memory area to restore universal access
    fn unlock(&mut self, area: Area);
    /// Detect if area is locked from an address
    fn is_available(&self, area: Area, lock_key: Option<Lock>) -> bool;
}

pub trait InternalLock<A: Into<u16>>: MemoryLock + FileOperation<A> {}
