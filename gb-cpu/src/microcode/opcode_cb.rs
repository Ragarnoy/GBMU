/// Opcode with the CB prefix
#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum OpcodeCB {
    /// `RLC B`: rotate `B` left, old 7 bit to `Carry`. 2m
    RlcB = 0x00,
    /// `RLC C`: rotate `C` left, old 7 bit to `Carry`. 2m
    RlcC = 0x01,
    /// `RLC D`: rotate `D` left, old 7 bit to `Carry`. 2m
    RlcD = 0x02,
    /// `RLC E`: rotate `E` left, old 7 bit to `Carry`. 2m
    RlcE = 0x03,
    /// `RLC H`: rotate `H` left, old 7 bit to `Carry`. 2m
    RlcH = 0x04,
    /// `RLC L`: rotate `L` left, old 7 bit to `Carry`. 2m
    RlcL = 0x05,
    /// `RLC (HL)`: rotate `(HL)` left, old 7 bit to `Carry`. 4m
    RlcHL = 0x06,
    /// `RLC A`: rotate `A` left, old 7 bit to `Carry`. 2m
    RlcA = 0x07,
    /// `RRC B`: rotate `B` right, old 0 bit to `Carry`. 2m
    RrcB = 0x08,
    /// `RRC C`: rotate `C` right, old 0 bit to `Carry`. 2m
    RrcC = 0x09,
    /// `RRC D`: rotate `D` right, old 0 bit to `Carry`. 2m
    RrcD = 0x0a,
    /// `RRC E`: rotate `E` right, old 0 bit to `Carry`. 2m
    RrcE = 0x0b,
    /// `RRC H`: rotate `H` right, old 0 bit to `Carry`. 2m
    RrcH = 0x0c,
    /// `RRC L`: rotate `L` right, old 0 bit to `Carry`. 2m
    RrcL = 0x0d,
    /// `RRC (HL)`: rotate `(HL)` right, old 0 bit to `Carry`. 4m
    RrcHL = 0x0e,
    /// `RRC A`: rotate `A` right, old 0 bit to `Carry`. 2m
    RrcA = 0x0f,
    /// `RL B`: rotate `B` left, old 7 bit to `Carry`. 2m
    RlB = 0x10,
    /// `RL C`: rotate `C` left, old 7 bit to `Carry`. 2m
    RlC = 0x11,
    /// `RL D`: rotate `D` left, old 7 bit to `Carry`. 2m
    RlD = 0x12,
    /// `RL E`: rotate `E` left, old 7 bit to `Carry`. 2m
    RlE = 0x13,
    /// `RL H`: rotate `H` left, old 7 bit to `Carry`. 2m
    RlH = 0x14,
    /// `RL L`: rotate `L` left, old 7 bit to `Carry`. 2m
    RlL = 0x15,
    /// `RL (HL)`: rotate `(HL)` left, old 7 bit to `Carry`. 4m
    RlHL = 0x16,
    /// `RL A`: rotate `A` left, old 7 bit to `Carry`. 2m
    RlA = 0x17,
    /// `RR B`: rotate `B` right, old 0 bit to `Carry`. 2m
    RrB = 0x18,
    /// `RR C`: rotate `C` right, old 0 bit to `Carry`. 2m
    RrC = 0x19,
    /// `RR D`: rotate `D` right, old 0 bit to `Carry`. 2m
    RrD = 0x1a,
    /// `RR E`: rotate `E` right, old 0 bit to `Carry`. 2m
    RrE = 0x1b,
    /// `RR H`: rotate `H` right, old 0 bit to `Carry`. 2m
    RrH = 0x1c,
    /// `RR L`: rotate `L` right, old 0 bit to `Carry`. 2m
    RrL = 0x1d,
    /// `RR (HL)`: rotate `(HL)` right, old 0 bit to `Carry`. 4m
    RrHL = 0x1e,
    /// `RR A`: rotate `A` right, old 0 bit to `Carry`. 2m
    RrA = 0x1f,
    /// `SLA B`: shift `B` left, old 7 bit to `Carry`. 2m
    SlaB = 0x20,
    /// `SLA C`: shift `C` left, old 7 bit to `Carry`. 2m
    SlaC = 0x21,
    /// `SLA D`: shift `D` left, old 7 bit to `Carry`. 2m
    SlaD = 0x22,
    /// `SLA E`: shift `E` left, old 7 bit to `Carry`. 2m
    SlaE = 0x23,
    /// `SLA H`: shift `H` left, old 7 bit to `Carry`. 2m
    SlaH = 0x24,
    /// `SLA L`: shift `L` left, old 7 bit to `Carry`. 2m
    SlaL = 0x25,
    /// `SLA (HL)`: shift `(HL)` left, old 7 bit to `Carry`. 4m
    SlaHL = 0x26,
    /// `SLA A`: shift `A` left, old 7 bit to `Carry`. 2m
    SlaA = 0x27,
    /// `SRA B`: shift `B` right, old 0 bit to `Carry`. 2m
    SraB = 0x28,
    /// `SRA C`: shift `C` right, old 0 bit to `Carry`. 2m
    SraC = 0x29,
    /// `SRA D`: shift `D` right, old 0 bit to `Carry`. 2m
    SraD = 0x2a,
    /// `SRA E`: shift `E` right, old 0 bit to `Carry`. 2m
    SraE = 0x2b,
    /// `SRA H`: shift `H` right, old 0 bit to `Carry`. 2m
    SraH = 0x2c,
    /// `SRA L`: shift `L` right, old 0 bit to `Carry`. 2m
    SraL = 0x2d,
    /// `SRA (HL)`: shift `(HL)` right, old 0 bit to `Carry`. 4m
    SraHL = 0x2e,
    /// `SRA A`: shift `A` right, old 0 bit to `Carry`. 2m
    SraA = 0x2f,
    /// `SWAP B`: swap upper & lower nibles of `B`. 2m
    SwapB = 0x30,
    /// `SWAP C`: swap upper & lower nibles of `C`. 2m
    SwapC = 0x31,
    /// `SWAP D`: swap upper & lower nibles of `D`. 2m
    SwapD = 0x32,
    /// `SWAP E`: swap upper & lower nibles of `E`. 2m
    SwapE = 0x33,
    /// `SWAP H`: swap upper & lower nibles of `H`. 2m
    SwapH = 0x34,
    /// `SWAP L`: swap upper & lower nibles of `L`. 2m
    SwapL = 0x35,
    /// `SWAP (HL)`: swap upper & lower nibles of `(HL)`. 4m
    SwapHL = 0x36,
    /// `SWAP A`: swap upper & lower nibles of `A`. 2m
    SwapA = 0x37,
    /// `SRL B`: shift `B` right. Most Smallest Bit set to 0. 2m
    SrlB = 0x38,
    /// `SRL C`: shift `C` right. Most Smallest Bit set to 0. 2m
    SrlC = 0x39,
    /// `SRL D`: shift `D` right. Most Smallest Bit set to 0. 2m
    SrlD = 0x3a,
    /// `SRL E`: shift `E` right. Most Smallest Bit set to 0. 2m
    SrlE = 0x3b,
    /// `SRL H`: shift `H` right. Most Smallest Bit set to 0. 2m
    SrlH = 0x3c,
    /// `SRL L`: shift `L` right. Most Smallest Bit set to 0. 2m
    SrlL = 0x3d,
    /// `SRL (HL)`: shift `(HL)` right. Most Smallest Bit set to 0. 4m
    SrlHL = 0x3e,
    /// `SRL A`: shift `A` right. Most Smallest Bit set to 0. 2m
    SrlA = 0x3f,

    /// `BIT0 B`: test if bit 0 is set on `B`. 2m
    Bit0B = 0x40,
    /// `BIT0 C`: test if bit 0 is set on `C`. 2m
    Bit0C = 0x41,
    /// `BIT0 D`: test if bit 0 is set on `D`. 2m
    Bit0D = 0x42,
    /// `BIT0 E`: test if bit 0 is set on `E`. 2m
    Bit0E = 0x43,
    /// `BIT0 H`: test if bit 0 is set on `H`. 2m
    Bit0H = 0x44,
    /// `BIT0 L`: test if bit 0 is set on `L`. 2m
    Bit0L = 0x45,
    /// `BIT0 (HL)`: test if bit 0 is set on `(HL)`. 4m
    Bit0HL = 0x46,
    /// `BIT0 A`: test if bit 0 is set on `A`. 2m
    Bit0A = 0x47,
    /// `BIT1 B`: test if bit 1 is set on `B`. 2m
    Bit1B = 0x48,
    /// `BIT1 C`: test if bit 1 is set on `C`. 2m
    Bit1C = 0x49,
    /// `BIT1 D`: test if bit 1 is set on `D`. 2m
    Bit1D = 0x4a,
    /// `BIT1 E`: test if bit 1 is set on `E`. 2m
    Bit1E = 0x4b,
    /// `BIT1 H`: test if bit 1 is set on `H`. 2m
    Bit1H = 0x4c,
    /// `BIT1 L`: test if bit 1 is set on `L`. 2m
    Bit1L = 0x4d,
    /// `BIT1 (HL)`: test if bit 1 is set on `(HL)`. 4m
    Bit1HL = 0x4e,
    /// `BIT1 A`: test if bit 1 is set on `A`. 2m
    Bit1A = 0x4f,
    /// `BIT2 B`: test if bit 2 is set on `B`. 2m
    Bit2B = 0x50,
    /// `BIT2 C`: test if bit 2 is set on `C`. 2m
    Bit2C = 0x51,
    /// `BIT2 D`: test if bit 2 is set on `D`. 2m
    Bit2D = 0x52,
    /// `BIT2 E`: test if bit 2 is set on `E`. 2m
    Bit2E = 0x53,
    /// `BIT2 H`: test if bit 2 is set on `H`. 2m
    Bit2H = 0x54,
    /// `BIT2 L`: test if bit 2 is set on `L`. 2m
    Bit2L = 0x55,
    /// `BIT2 (HL)`: test if bit 2 is set on `(HL)`. 4m
    Bit2HL = 0x56,
    /// `BIT2 A`: test if bit 2 is set on `A`. 2m
    Bit2A = 0x57,
    /// `BIT3 B`: test if bit 3 is set on `B`. 2m
    Bit3B = 0x58,
    /// `BIT3 C`: test if bit 3 is set on `C`. 2m
    Bit3C = 0x59,
    /// `BIT3 D`: test if bit 3 is set on `D`. 2m
    Bit3D = 0x5a,
    /// `BIT3 E`: test if bit 3 is set on `E`. 2m
    Bit3E = 0x5b,
    /// `BIT3 H`: test if bit 3 is set on `H`. 2m
    Bit3H = 0x5c,
    /// `BIT3 L`: test if bit 3 is set on `L`. 2m
    Bit3L = 0x5d,
    /// `BIT3 (HL)`: test if bit 3 is set on `(HL)`. 4m
    Bit3HL = 0x5e,
    /// `BIT3 A`: test if bit 3 is set on `A`. 2m
    Bit3A = 0x5f,
    /// `BIT4 B`: test if bit 4 is set on `B`. 2m
    Bit4B = 0x60,
    /// `BIT4 C`: test if bit 4 is set on `C`. 2m
    Bit4C = 0x61,
    /// `BIT4 D`: test if bit 4 is set on `D`. 2m
    Bit4D = 0x62,
    /// `BIT4 E`: test if bit 4 is set on `E`. 2m
    Bit4E = 0x63,
    /// `BIT4 H`: test if bit 4 is set on `H`. 2m
    Bit4H = 0x64,
    /// `BIT4 L`: test if bit 4 is set on `L`. 2m
    Bit4L = 0x65,
    /// `BIT4 (HL)`: test if bit 4 is set on `(HL)`. 4m
    Bit4HL = 0x66,
    /// `BIT4 A`: test if bit 4 is set on `A`. 2m
    Bit4A = 0x67,
    /// `BIT5 B`: test if bit 5 is set on `B`. 2m
    Bit5B = 0x68,
    /// `BIT5 C`: test if bit 5 is set on `C`. 2m
    Bit5C = 0x69,
    /// `BIT5 D`: test if bit 5 is set on `D`. 2m
    Bit5D = 0x6a,
    /// `BIT5 E`: test if bit 5 is set on `E`. 2m
    Bit5E = 0x6b,
    /// `BIT5 H`: test if bit 5 is set on `H`. 2m
    Bit5H = 0x6c,
    /// `BIT5 L`: test if bit 5 is set on `L`. 2m
    Bit5L = 0x6d,
    /// `BIT5 (HL)`: test if bit 5 is set on `(HL)`. 4m
    Bit5HL = 0x6e,
    /// `BIT5 A`: test if bit 5 is set on `A`. 2m
    Bit5A = 0x6f,
    /// `BIT6 B`: test if bit 6 is set on `B`. 2m
    Bit6B = 0x70,
    /// `BIT6 C`: test if bit 6 is set on `C`. 2m
    Bit6C = 0x71,
    /// `BIT6 D`: test if bit 6 is set on `D`. 2m
    Bit6D = 0x72,
    /// `BIT6 E`: test if bit 6 is set on `E`. 2m
    Bit6E = 0x73,
    /// `BIT6 H`: test if bit 6 is set on `H`. 2m
    Bit6H = 0x74,
    /// `BIT6 L`: test if bit 6 is set on `L`. 2m
    Bit6L = 0x75,
    /// `BIT6 (HL)`: test if bit 6 is set on `(HL)`. 4m
    Bit6HL = 0x76,
    /// `BIT6 A`: test if bit 6 is set on `A`. 2m
    Bit6A = 0x77,
    /// `BIT7 B`: test if bit 7 is set on `B`. 2m
    Bit7B = 0x78,
    /// `BIT7 C`: test if bit 7 is set on `C`. 2m
    Bit7C = 0x79,
    /// `BIT7 D`: test if bit 7 is set on `D`. 2m
    Bit7D = 0x7a,
    /// `BIT7 E`: test if bit 7 is set on `E`. 2m
    Bit7E = 0x7b,
    /// `BIT7 H`: test if bit 7 is set on `H`. 2m
    Bit7H = 0x7c,
    /// `BIT7 L`: test if bit 7 is set on `L`. 2m
    Bit7L = 0x7d,
    /// `BIT7 (HL)`: test if bit 7 is set on `(HL)`. 4m
    Bit7HL = 0x7e,
    /// `BIT7 A`: test if bit 7 is set on `A`. 2m
    Bit7A = 0x7f,
}
