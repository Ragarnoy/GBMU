use crate::address_bus::Error;

trait AddressBus {
    /// read `N` into the bus where `N: u8 | u16`
    fn read<N>(address: u16) -> Result<N, Error>;
    /// write `N` into the bus where `N: u8 | u16`
    fn write<N>(address: u16, data: N) -> Result<(), Error>;
}
