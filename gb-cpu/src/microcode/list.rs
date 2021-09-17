#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    /// `NOP`: do nothing, 1m
    Nop = 0x00,
    /// `LD BC, u16`: load value `u16` into `BC`, 3m
    LdBC16 = 0x01,
    /// `LD (BC), A`: load value `A` into `(BC)`, 2m
    LdBCAInd = 0x02,
    /// `INC BC`: increment `BC`, 2m
    IncBC = 0x03,
    /// `INC B`: increment `B`, 1m
    IncB = 0x04,
    /// `DEC B`: decrement `B`, 1m
    DecB = 0x05,
    /// `LD B, u8`: load value `u8` into `B`, 2m
    LdB = 0x06,
    /// `RLCA`: rotate `A` left, 1m
    RlcA = 0x07,
    /// `LD (u16), SP`: load value `SP` into `(u16)`, 5m
    Ld16SPInd = 0x08,
    /// `ADD HL, BC`: add `BC` to `HL`, 2m
    AddHLBC = 0x09,
    /// `LD A, (BC)`: load value `(BC)` into `A`, 2m
    LdABCInd = 0x0a,
    /// `DEC BC`: decrement `BC`, 2m
    DecBC = 0x0b,
    /// `INC BC`: increment `BC`, 2m
    IncC = 0x0c,
    /// `DEC C`: decrement `C`, 1m
    DecC = 0x0d,
    /// `LD C, u8`: load value `u8` into `C`, 2m
    LdC8 = 0x0e,
    /// `RRCA`: rotate `A` right, 1m
    RrcA = 0x0f,
    /// `STOP`: stop, 1m
    Stop = 0x10,
}
