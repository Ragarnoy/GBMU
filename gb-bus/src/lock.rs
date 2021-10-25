use crate::Area;

#[derive(Eq, PartialEq)]
pub enum Lock {
    Ppu,
}

pub trait MemoryLock {
    /// Lock a memory area for exclusive access
    fn lock(&mut self, area: Area, lock: Lock);
    /// unlock a memory area to restore universal access
    fn unlock(&mut self, area: Area);
    /// Detect if area is locked from an address
    fn is_available(&self, address: u16, lock_key: Option<Lock>) -> bool;
}
