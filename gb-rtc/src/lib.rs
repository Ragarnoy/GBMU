/// This trait allow to read the RTC registers
pub trait ReadRtcRegisters {
    fn seconds(&self) -> u8;
    fn minutes(&self) -> u8;
    fn hours(&self) -> u8;
    fn lower_days(&self) -> u8;
    fn upper_days(&self) -> bool;

    /// Combination of `lower_days` and `upper_days`
    fn days(&self) -> u16;

    /// Register used to halt the clock
    ///
    /// - 0: running
    /// - 1: halted
    fn halted(&self) -> bool;

    /// The day counter carry when the day counter overflow
    /// Need to be reset by the app
    fn day_counter_carry(&self) -> bool;

    /// Return the control bitfields
    ///
    /// - Bit 0: Upper day counter
    /// - Bit 6: Halt
    /// - Bit 7: Day Counter Carry
    fn control(&self) -> u8;
}

/// This trait allow to modify the RTC registers
pub trait WriteRtcRegisters: Sized {
    fn set_seconds(&self, seconds: u8) -> Option<Self>;
    fn set_minutes(&self, minutes: u8) -> Option<Self>;
    fn set_hours(&self, hours: u8) -> Option<Self>;
    fn set_lower_days(&self, days: u8) -> Option<Self>;

    /// Set the one bit used for the upper days
    fn set_upper_days(&self, upper_days: bool) -> Option<Self>;
    fn set_halted(&self, halted: bool) -> Option<Self>;
    fn set_day_counter_carry(&self, carry: bool) -> Option<Self>;

    /// Set the control bitfields
    ///
    /// - Bit 0: Upper day counter
    /// - Bit 6: Halt
    /// - Bit 7: Day Counter Carry
    fn set_control(&self, control: u8) -> Option<Self>;
}
