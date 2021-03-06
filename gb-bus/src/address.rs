#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Address contain the relative and absolute address
pub struct Addr<A> {
    /// relative address into the current area of the address bus
    pub relative: u16,

    /// absolute address used in the address bus
    pub absolute: u16,

    pub area: A,
}

impl<A> Addr<A> {
    pub fn new(area: A, relative_addr: u16, absolute_addr: u16) -> Self {
        Self {
            relative: relative_addr,
            absolute: absolute_addr,
            area,
        }
    }

    /// Create an address for single byte registry
    pub fn byte_reg(area: A, absolute_addr: u16) -> Self {
        Self {
            relative: 0,
            absolute: absolute_addr,
            area,
        }
    }

    /// Create an Address from an absolute address and an offset
    ///
    /// ```
    /// # use gb_bus::{address::Addr, Area};
    /// let pos = Addr::from_offset(Area::Rom, 0x42, 0x10);
    ///
    /// assert_eq!(pos.absolute, 0x42);
    /// assert_eq!(pos.relative, 0x32);
    /// ```
    pub fn from_offset(area: A, addr: u16, offset: u16) -> Self {
        Self::new(area, addr - offset, addr)
    }
}

impl<A: Copy + Clone> crate::file_operation::Address<A> for Addr<A> {
    fn get_address(&self) -> usize {
        self.relative as usize
    }

    fn area_type(&self) -> A {
        self.area
    }
}

impl<T> From<Addr<T>> for u16 {
    fn from(a: Addr<T>) -> Self {
        a.absolute
    }
}
