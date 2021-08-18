use crate::address_bus::Area;

trait Address {
    /// Return the relative address in the current area
    fn get_address() -> usize;

    /// Return the current area type
    fn area_type() -> Area;
}
