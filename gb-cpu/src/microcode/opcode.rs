#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    /// `NOP`: do nothing, 1m
    Nop = 0x00,
    /// `LD BC, u16`: load `u16` into `BC`, 3m
    LdBC16 = 0x01,
    /// `LD (BC), A`: load `A` into `(BC)`, 2m
    LdBCA = 0x02,
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
    Ld16SP = 0x08,
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

    /// `JR NZ, i8`: relative jump to `i8` when `Zero` is not set, 2-3m
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

    /// `JR NC, i8`: relative jump to `i8` when `Carry` is not set, 2-3m
    JrNC8 = 0x30,
    /// `LD SP, u16`: load `u16` into `SP`, 3m
    LdSP16 = 0x31,
    /// `LDD (HL), A`: load `A` into `HL` then decrement `HL`, 2m
    LddHLA = 0x32,
    /// `INC SP`: increment `SP`, 2m
    IncSP = 0x33,
    /// `INC (HL)`: increment `(HL)`, 3m
    IncHLind = 0x34,
    /// `DEC (HL)`: decrement `(HL)`, 3m
    DecHLind = 0x35,
    /// `LD (HL), u8`: load `u8` into `(HL)`, 3m
    LdHL8 = 0x36,
    /// `SCF`: set `Carry`, 1m
    Scf = 0x37,
    /// `JR C, i8`: relative jump to `i8` when `Carry`, 2-3m
    JrC8 = 0x38,
    /// `ADD HL, SP`: add `SP` to `HL`, 2m
    AddHLSP = 0x39,
    /// `LDD A, (HL)`: load `(HL)` into `A` then decrement `HL`, 2m
    LddAHL = 0x3a,
    /// `DEC SP`: decrement `SP`, 2m
    DecSP = 0x3b,
    /// `INC A`: increment `A`, 1m
    IncA = 0x3c,
    /// `DEC A`: decrement `A`, 1m
    DecA = 0x3d,
    /// `LD A, u8`: load `u8` into `A`, 2m
    LdA8 = 0x3e,
    /// `CCF`: toggle `Carry`, 1m
    Ccf = 0x3f,

    /// `LD B, B`: load `B` into `B`, 1m
    LdBB = 0x40,
    /// `LD B, C`: load `C` into `B`, 1m
    LdBC = 0x41,
    /// `LD B, D`: load `D` into `B`, 1m
    LdBD = 0x42,
    /// `LD B, E`: load `E` into `B`, 1m
    LdBE = 0x43,
    /// `LD B, H`: load `H` into `B`, 1m
    LdBH = 0x44,
    /// `LD B, L`: load `L` into `B`, 1m
    LdBL = 0x45,
    /// `LD B, (HL)`: load `(HL)` into `B`, 2m
    LdBHL = 0x46,
    /// `LD B, A`: load `A` into `B`, 1m
    LdBA = 0x47,
    /// `LD C, B`: load `B` into `C`, 1m
    LdCB = 0x48,
    /// `LD C, C`: load `C` into `C`, 1m
    LdCC = 0x49,
    /// `LD C, D`: load `D` into `C`, 1m
    LdCD = 0x4a,
    /// `LD C, E`: load `E` into `C`, 1m
    LdCE = 0x4b,
    /// `LD C, H`: load `H` into `C`, 1m
    LdCH = 0x4c,
    /// `LD C, L`: load `L` into `C`, 1m
    LdCL = 0x4d,
    /// `LD C, (HL)`: load `(HL)` into `C`, 2m
    LdCHL = 0x4e,
    /// `LD C, A`: load `A` into `C`, 1m
    LdCA = 0x4f,

    /// `LD D, B`: load `B` into `D`, 1m
    LdDB = 0x50,
    /// `LD D, C`: load `C` into `D`, 1m
    LdDC = 0x51,
    /// `LD D, D`: load `D` into `D`, 1m
    LdDD = 0x52,
    /// `LD D, E`: load `E` into `D`, 1m
    LdDE = 0x53,
    /// `LD D, H`: load `H` into `D`, 1m
    LdDH = 0x54,
    /// `LD D, L`: load `L` into `D`, 1m
    LdDL = 0x55,
    /// `LD D, (HL)`: load `(HL)` into `D`, 2m
    LdDHL = 0x56,
    /// `LD D, A`: load `A` into `D`, 1m
    LdDA = 0x57,
    /// `LD E, B`: load `B` into `E`, 1m
    LdEB = 0x58,
    /// `LD E, C`: load `C` into `E`, 1m
    LdEC = 0x59,
    /// `LD E, D`: load `D` into `E`, 1m
    LdED = 0x5a,
    /// `LD E, E`: load `E` into `E`, 1m
    LdEE = 0x5b,
    /// `LD E, H`: load `H` into `E`, 1m
    LdEH = 0x5c,
    /// `LD E, L`: load `L` into `E`, 1m
    LdEL = 0x5d,
    /// `LD E, (HL)`: load `(HL)` into `E`, 2m
    LdEHL = 0x5e,
    /// `LD E, A`: load `A` into `E`, 1m
    LdEA = 0x5f,

    /// `LD H, B`: load `B` into `H`, 1m
    LdHB = 0x60,
    /// `LD H, C`: load `C` into `H`, 1m
    LdHC = 0x61,
    /// `LD H, D`: load `D` into `H`, 1m
    LdHD = 0x62,
    /// `LD H, E`: load `E` into `H`, 1m
    LdHE = 0x63,
    /// `LD H, H`: load `H` into `H`, 1m
    LdHH = 0x64,
    /// `LD H, L`: load `L` into `H`, 1m
    LdHL = 0x65,
    /// `LD H, (HL)`: load `(HL)` into `H`, 2m
    LdHHL = 0x66,
    /// `LD H, A`: load `A` into `H`, 1m
    LdHA = 0x67,
    /// `LD L, B`: load `B` into `L`, 1m
    LdLB = 0x68,
    /// `LD L, C`: load `C` into `L`, 1m
    LdLC = 0x69,
    /// `LD L, D`: load `D` into `L`, 1m
    LdLD = 0x6a,
    /// `LD L, E`: load `E` into `L`, 1m
    LdLE = 0x6b,
    /// `LD L, H`: load `H` into `L`, 1m
    LdLH = 0x6c,
    /// `LD L, L`: load `L` into `L`, 1m
    LdLL = 0x6d,
    /// `LD L, (HL)`: load `(HL)` into `L`, 2m
    LdLHL = 0x6e,
    /// `LD L, A`: load `A` into `L`, 1m
    LdLA = 0x6f,

    /// `LD (HL), B`: load `B` into `(HL)`, 2m
    LdHLB = 0x70,
    /// `LD (HL), C`: load `C` into `(HL)`, 2m
    LdHLC = 0x71,
    /// `LD (HL), D`: load `D` into `(HL)`, 2m
    LdHLD = 0x72,
    /// `LD (HL), E`: load `E` into `(HL)`, 2m
    LdHLE = 0x73,
    /// `LD (HL), H`: load `H` into `(HL)`, 2m
    LdHLH = 0x74,
    /// `LD (HL), L`: load `L` into `(HL)`, 2m
    LdHLL = 0x75,
    /// `HALT`: power down the cpu until an interrupt occurs, 1m
    Halt = 0x76,
    /// `LD (HL), A`: load `A` into `(HL)`, 2m
    LdHLA = 0x77,
    /// `LD A, B`: load `B` into `A`, 1m
    LdAB = 0x78,
    /// `LD A, C`: load `C` into `A`, 1m
    LdAC = 0x79,
    /// `LD A, D`: load `D` into `A`, 1m
    LdAD = 0x7a,
    /// `LD A, E`: load `E` into `A`, 1m
    LdAE = 0x7b,
    /// `LD A, H`: load `H` into `A`, 1m
    LdAH = 0x7c,
    /// `LD A, L`: load `L` into `A`, 1m
    LdAL = 0x7d,
    /// `LD A, (HL)`: load `(HL)` into `A`, 2m
    LdAHL = 0x7e,
    /// `LD A, A`: load `A` into `A`, 1m
    LdAA = 0x7f,

    /// `ADD A, B`: add `B` to `A`, 1m
    AddAB = 0x80,
    /// `ADD A, C`: add `C` to `A`, 1m
    AddAC = 0x81,
    /// `ADD A, D`: add `D` to `A`, 1m
    AddAD = 0x82,
    /// `ADD A, E`: add `E` to `A`, 1m
    AddAE = 0x83,
    /// `ADD A, H`: add `H` to `A`, 1m
    AddAH = 0x84,
    /// `ADD A, L`: add `L` to `A`, 1m
    AddAL = 0x85,
    /// `ADD A, (HL)`: add `(HL)` to `A`, 2m
    AddAHL = 0x86,
    /// `ADD A, A`: add `A` to `A`, 1m
    AddAA = 0x87,
    /// `ADC A, B`: add `B + Carry` to `A`, 1m
    AdcAB = 0x88,
    /// `ADC A, C`: add `C + Carry` to `A`, 1m
    AdcAC = 0x89,
    /// `ADC A, D`: add `D + Carry` to `A`, 1m
    AdcAD = 0x8a,
    /// `ADC A, E`: add `E + Carry` to `A`, 1m
    AdcAE = 0x8b,
    /// `ADC A, H`: add `H + Carry` to `A`, 1m
    AdcAH = 0x8c,
    /// `ADC A, L`: add `L + Carry` to `A`, 1m
    AdcAL = 0x8d,
    /// `ADC A, (HL)`: add `(HL) + Carry` to `A`, 2m
    AdcAHL = 0x8e,
    /// `ADC A, A`: add `A + Carry` to `A`, 1m
    AdcAA = 0x8f,

    /// `SUB A, B`: sub `B` to `A`, 1m
    SubAB = 0x90,
    /// `SUB A, C`: sub `C` to `A`, 1m
    SubAC = 0x91,
    /// `SUB A, D`: sub `D` to `A`, 1m
    SubAD = 0x92,
    /// `SUB A, E`: sub `E` to `A`, 1m
    SubAE = 0x93,
    /// `SUB A, H`: sub `H` to `A`, 1m
    SubAH = 0x94,
    /// `SUB A, L`: sub `L` to `A`, 1m
    SubAL = 0x95,
    /// `SUB A, (HL)`: sub `(HL)` to `A`, 2m
    SubAHL = 0x96,
    /// `SUB A, A`: sub `A` to `A`, 1m
    SubAA = 0x97,
    /// `SBC A, B`: sub `B + Carry` to `A`, 1m
    SbcAB = 0x98,
    /// `SBC A, C`: sub `C + Carry` to `A`, 1m
    SbcAC = 0x99,
    /// `SBC A, D`: sub `D + Carry` to `A`, 1m
    SbcAD = 0x9a,
    /// `SBC A, E`: sub `E + Carry` to `A`, 1m
    SbcAE = 0x9b,
    /// `SBC A, H`: sub `H + Carry` to `A`, 1m
    SbcAH = 0x9c,
    /// `SBC A, L`: sub `L + Carry` to `A`, 1m
    SbcAL = 0x9d,
    /// `SBC A, (HL)`: sub `(HL) + Carry` to `A`, 2m
    SbcAHL = 0x9e,
    /// `SBC A, A`: sub `A + Carry` to `A`, 1m
    SbcAA = 0x9f,

    /// `AND A, B`: logic `A = B & A`, 1m
    AndAB = 0xa0,
    /// `AND A, C`: logic `A = C & A`, 1m
    AndAC = 0xa1,
    /// `AND A, D`: logic `A = D & A`, 1m
    AndAD = 0xa2,
    /// `AND A, E`: logic `A = E & A`, 1m
    AndAE = 0xa3,
    /// `AND A, H`: logic `A = H & A`, 1m
    AndAH = 0xa4,
    /// `AND A, L`: logic `A = L & A`, 1m
    AndAL = 0xa5,
    /// `AND A, (HL)`: logical `A = (HL) & A`, 2m
    AndAHL = 0xa6,
    /// `AND A, A`: logic `A = A & A`, 1m
    AndAA = 0xa7,
    /// `XOR A, B`: logic `A = B ^ A`, 1m
    XorAB = 0xa8,
    /// `XOR A, C`: logic `A = C ^ A`, 1m
    XorAC = 0xa9,
    /// `XOR A, D`: logic `A = D ^ A`, 1m
    XorAD = 0xaa,
    /// `XOR A, E`: logic `A = E ^ A`, 1m
    XorAE = 0xab,
    /// `XOR A, H`: logic `A = H ^ A`, 1m
    XorAH = 0xac,
    /// `XOR A, L`: logic `A = L ^ A`, 1m
    XorAL = 0xad,
    /// `XOR A, (HL)`: logical `A = (HL) ^ A`, 2m
    XorAHL = 0xae,
    /// `XOR A, A`: logic `A = A ^ A`, 1m
    XorAA = 0xaf,

    /// `OR A, B`: logic `A = B | A`, 1m
    OrAB = 0xb0,
    /// `OR A, C`: logic `A = C | A`, 1m
    OrAC = 0xb1,
    /// `OR A, D`: logic `A = D | A`, 1m
    OrAD = 0xb2,
    /// `OR A, E`: logic `A = E | A`, 1m
    OrAE = 0xb3,
    /// `OR A, H`: logic `A = H | A`, 1m
    OrAH = 0xb4,
    /// `OR A, L`: logic `A = L | A`, 1m
    OrAL = 0xb5,
    /// `OR A, (HL)`: logical `A = (HL) | A`, 2m
    OrAHL = 0xb6,
    /// `OR A, A`: logic `A = A | A`, 1m
    OrAA = 0xb7,
    /// `CP A, B`: compare A with B, 1m
    CpAB = 0xb8,
    /// `CP A, C`: compare A with C, 1m
    CpAC = 0xb9,
    /// `CP A, D`: compare A with D, 1m
    CpAD = 0xba,
    /// `CP A, E`: compare A with E, 1m
    CpAE = 0xbb,
    /// `CP A, H`: compare A with H, 1m
    CpAH = 0xbc,
    /// `CP A, L`: compare A with L, 1m
    CpAL = 0xbd,
    /// `CP A, (HL)`: compare A with (HL), 2m
    CpAHL = 0xbe,
    /// `CP A, A`: compare A with A, 1m
    CpAA = 0xbf,

    /// `RET NZ`: Pop two bytes from stack & jump to that address if `Zero` is not set. 2-5m
    RetNz = 0xc0,
    /// `POP BC`: Pop two bytes off the stack into `BC`. 3m
    PopBc = 0xc1,
    /// `JP NZ, u16`: Absolute jump to `u16` if `NZ` is set. 3-4m
    JpNz = 0xc2,
    /// `JP u16`: Absolute jump to `u16`. 4m
    Jp = 0xc3,
    /// `CALL NZ, u16`: When `NZ` is set: Push the address of the next instruction then jump to `u16`. 3-6m
    CallNz = 0xc4,
    /// `PUSH BC`: push `BC` into the stack. 4m
    PushBc = 0xc5,
    /// `ADD A, u8`: add `u8` to `A`. 2m
    AddA8 = 0xc6,
    /// `RST 00`: push current address onto the stack the jump to `0x00`. 4m
    Rst00 = 0xc7,
    /// `RET Z`: Pop two bytes from stack & jump to that address if `Zero` is set. 2-5m
    RetZ = 0xc8,
    /// `RET`: Pop two bytes from the stack the jump to that address. 4m
    Ret = 0xc9,
    /// `JP Z, u16`: Absolute jump to `u16` if `Z` is set. 3-4m
    JpZ16 = 0xca,
    /// Opcode with CB prefix see `microcode::opcode_cb::OpcodeCB`
    PrefixCb = 0xcb,
    /// `CALL Z, u16`: When `Z` is set: Push the address of the next instruction then jump to `u16`. 3-6m
    CallZ16 = 0xcc,
    /// `CALL u16`: push address of the next instruction onto the stack then jump to `u16`. 6m
    Call16 = 0xcd,
    /// `ADC A, u8`: add `u8 + Carry` to `A`. 2m
    AdcA8 = 0xce,
    /// `RST 08`: push current address onto the stack the jump to `0x08`. 4m
    Rst08 = 0xcf,
    /// `RET NC`: Pop two bytes from stack & jump to that address if `Carry` is not set. 2-5m
    RetNc = 0xd0,
    /// `POP DE`: Pop two bytes off the stack into `DE`. 3m
    PopDe = 0xd1,
    /// `JP NC, u16`: Absolute jump to `u16` if `NC` is set. 3-4m
    JpNc16 = 0xd2,
    /// `CALL NC, u16`: When `NC` is set: Push the address of the next instruction then jump to `u16`. 3-6m
    CallNc16 = 0xd4,
    /// `PUSH DE`: push `DE` into the stack. 4m
    PushDe = 0xd5,
    /// `SUB A, u8`: sub `u8` to `A`. 2m
    SubA8 = 0xd6,
    /// `RST 10`: push current address onto the stack the jump to `0x10`. 4m
    Rst10 = 0xd7,
    /// `RET C`: Pop two bytes from stack & jump to that address if `Carry` is set. 2-5m
    RetC = 0xd8,
    /// `RETI`: Pop two bytes from the stack then jump to that address and the enable interrupts. 4m
    Reti = 0xd9,
    /// `JP C, u16`: Absolute jump to `u16` if `C` is set. 3-4m
    JpC16 = 0xda,
    /// `CALL C, u16`: When `C` is set: Push the address of the next instruction then jump to `u16`. 3-6m
    CallC16 = 0xdc,
    /// `SBC A, u8`: sub `u8 + Carry` to `A`. 2m
    SbcA8 = 0xde,
    /// `RST 18`: push current address onto the stack the jump to `0x18`. 4m
    Rst18 = 0xdf,
    /// `LDH (u8), A`: load `A` into `(0xFF00 + u8)`. 3m
    Ldh8A = 0xe0,
    /// `POP HL`: Pop two bytes off the stack into `HL`. 3m
    PopHl = 0xe1,
    /// `LDH (C), A`: load `A` into `(0xFF00 + C). 2m
    LdhCA = 0xe2,
    /// `PUSH HL`: push `HL` into the stack. 4m
    PushHl = 0xe5,
    /// `AND A, u8`: logical `A = u8 & A`. 2m
    AndA8 = 0xe6,
    /// `RST 20`: push current address onto the stack the jump to `0x20`. 4m
    Rst20 = 0xe7,
    /// `ADD SP, i8`: add `i8` to `SP`. 4m
    AddSp8 = 0xe8,
    /// `JP HL`: Absolute jump to `HL`. 1m
    JpHl = 0xe9,
    /// `LD (u16),A`: load `A` into `(u16)`. 4m
    Ld16A = 0xea,
    /// `XOR A, u8`: logical `A = u8 ^ A`. 2m
    XorA8 = 0xee,
    /// `RST 28`: push current address onto the stack the jump to `0x28`. 4m
    Rst28 = 0xef,
    /// `LDH A,(u8)`: load `(u8)` into `A`. 3m
    LdhA8 = 0xf0,
    /// `POP AF`: Pop two bytes off the stack into `AF`. 3m
    PopAf = 0xf1,
    /// `LDH A,(C)`: load `(0xFF00 + C)` into `A`. 2m
    LdhAC = 0xf2,
    /// `DI`: Disable interrups after the end of the next instruction. 1m
    Di = 0xf3,
    /// `PUSH AF`: push `AF` into the stack. 4m
    PushAf = 0xf5,
    /// `OR A, u8`: logical `A = u8 | A`. 2m
    OrA8 = 0xf6,
    /// `RST 30`: push current address onto the stack the jump to `0x30`. 4m
    Rst30 = 0xf7,
    /// `LDHL SP, i8`: put `SP + i8` into `HL`. 3m
    LdhlSp8 = 0xf8,
    /// `LD SP, HL`: load `HL` inot `SP`. 2m
    LdSpHl = 0xf9,
    /// `LD A,(u16)`: load `(u16)` into `A`. 4m
    LdA16 = 0xfa,
    /// `EI`: Enable interrups after the end of the next instruction. 1m
    Ei = 0xfb,
    /// `CP A, u8`: Compare `A` to `u8`. 2m
    CpA8 = 0xfe,
    /// `RST 38`: push current address onto the stack the jump to `0x38`. 4m
    Rst38 = 0xff,
}
