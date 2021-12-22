#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Address contain the relative and absolute address
pub struct Address<A> {
    /// relative address into the current area of the address bus
    pub relative: u16,

    /// absolute address used in the address bus
    pub absolute: u16,

    pub area: A,
}

impl<A> Address<A> {
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

    /// Create an Address from an absolute adress and an offset
    ///
    /// ```
    /// # use gb_bus::{address::Address, Area};
    /// let pos = Address::from_offset(Area::Rom, 0x42, 0x10);
    ///
    /// assert_eq!(pos.absolute, 0x42);
    /// assert_eq!(pos.relative, 0x32);
    /// ```
    pub fn from_offset(area: A, addr: u16, offset: u16) -> Self {
        Self::new(area, addr - offset, addr)
    }
}

impl<A: Copy + Clone> From<Address<A>> for u16 {
    fn from(addr: Address<A>) -> Self {
        addr.absolute
    }
}

impl<A: Copy + Clone> crate::file_operation::Address<A> for Address<A> {
    fn get_address(&self) -> usize {
        self.relative as usize
    }

    fn area_type(&self) -> A {
        self.area
    }
}
