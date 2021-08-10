#[derive(Debug)]
pub enum _8Bits {
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum _16Bits {
    SP,
    PC,
    BC,
    DE,
    HL,
}

#[derive(Debug)]
pub enum Flags {
    /// Zero flag
    /// This flag is set when :
    /// - the result of a math op is zero
    /// - `Cmp` OP match 2 values
    Z,

    /// Substract Flag
    /// This flag is set when the last math instruction was a substraction
    N,

    /// Half Carry Flag
    /// This flag is set when a carry occurred in the lower nibble of the last math OP
    H,

    /// Carry Flag
    /// This flag is set when :
    /// - a carry occurred in the last math OP
    /// - Reg A is the smaller value when doing a `Cmp` OP
    C,
}

