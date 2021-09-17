#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    /// `NOP`: do nothing, 1m
    Nop = 0x00,
    /// `LD BC, u16`: load `u16` into `BC`, 3m
    LdBC16 = 0x01,
    /// `LD (BC), A`: load `A` into `(BC)`, 2m
    LdBCAInd = 0x02,
    /// `INC BC`: increment `BC`, 2m
    IncBC = 0x03,
    /// `INC B`: increment `B`, 1m
    IncB = 0x04,
    /// `DEC B`: decrement `B`, 1m
    DecB = 0x05,
    /// `LD B, u8`: load `u8` into `B`, 2m
    LdB = 0x06,
    /// `RLCA`: rotate `A` left, 1m
    RlcA = 0x07,
    /// `LD (u16), SP`: load `SP` into `(u16)`, 5m
    Ld16SPInd = 0x08,
    /// `ADD HL, BC`: add `BC` to `HL`, 2m
    AddHLBC = 0x09,
    /// `LD A, (BC)`: load `(BC)` into `A`, 2m
    LdABCInd = 0x0a,
    /// `DEC BC`: decrement `BC`, 2m
    DecBC = 0x0b,
    /// `INC BC`: increment `BC`, 2m
    IncC = 0x0c,
    /// `DEC C`: decrement `C`, 1m
    DecC = 0x0d,
    /// `LD C, u8`: load `u8` into `C`, 2m
    LdC8 = 0x0e,
    /// `RRCA`: rotate `A` right, 1m
    RrcA = 0x0f,

    /// `STOP`: stop, 1m
    /// The next byte should be `00`
    Stop = 0x10,
    /// `LD DE, u16`: load `u16` into `DE`, 3m
    LdDE16 = 0x11,
    /// `LD (DE), A`: load `A` into `(DE)`, 2m
    LdDEAInd = 0x12,
    /// `INC DE`: increment `DE`, 2m
    IncDE = 0x13,
    /// `INC D`: increment `D`, 1m
    IncD = 0x14,
    /// `DEC D`: decrement `D`, 1m
    DecD = 0x15,
    /// `LD D, u8`: load `u8` into `D`, 2m
    LdD8 = 0x16,
    /// `RLA`: rotate `A` left, 1m
    Rla = 0x17,
    /// `JR i8`: relative jump to `i8`, 3m
    Jr8 = 0x18,
    /// `ADD HL, DE`: add `DE` to `HL`, 2m
    AddHLDE = 0x19,
    /// `LD A, (DE)`: load `(DE)` into `A`, 2m
    LdADEInd = 0x1a,
    /// `DEC DE`: decrement `DE`, 2m
    DecDE = 0x1b,
    /// `INC E`: increment `E`, 1m
    IncE = 0x1c,
    /// `DEC E`: decrement `E`, 1m
    DecE = 0x1d,
    /// `LD E, u8`: load `u8` into `E`, 2m
    LdE8 = 0x1e,
    /// `RRA`: rotate `A` right, 1m
    Rra = 0x1f,

    /// `JR NZ,i8`: relative jump to `i8` when `Zero` is not set, 2-3m
    JrNZ8 = 0x20,
    /// `LD HL, u16`: load `u16` into `HL`, 3m
    LdHL16 = 0x21,
    /// `LDI (HL), A`: load `A` into `(HL)` then increment `HL`, 2m
    LdiHLA = 0x22,
    /// `INC HL`: increment `HL`, 2m
    IncHL = 0x23,
    /// `INC H`: increment `H`, 1m
    IncH = 0x24,
    /// `DEC H`: decrement `H`, 1m
    DecH = 0x25,
    /// `LD H, u8`: load `u8` into `H`, 2m
    LdH8 = 0x26,
    /// `DAA`: decimal ajust `A`, 1m
    Daa = 0x27,
    /// `JR Z, i8`: relative jump to `i8` when `Zero` is set, 2-3m
    JrZ8 = 0x28,
    /// `ADD HL, HL`: add `HL` to `HL`
    AddHLHL = 0x29,
    /// `LDI A, (HL)`: load `(HL)` into `A` then increment `HL`, 2m
    LdiAHL = 0x2a,
    /// `DEC HL`: decrement `HL`, 2m
    DecHL = 0x2b,
    /// `INC L`: increment `L`, 1m
    IncL = 0x2c,
    /// `DEC L`: decrement `L`, 1m
    DecL = 0x2d,
    /// `LD L, u8`: load `u8` into `L`, 2m
    LdL8 = 0x2e,
    /// `CPL`: flip all bits of `A`, 1m
    Cpl = 0x2f,
}
