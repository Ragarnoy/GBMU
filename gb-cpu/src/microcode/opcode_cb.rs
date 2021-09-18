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

    /// `BIT 0, B`: test if bit 0 is set on `B`. 2m
    Bit0B = 0x40,
    /// `BIT 0, C`: test if bit 0 is set on `C`. 2m
    Bit0C = 0x41,
    /// `BIT 0, D`: test if bit 0 is set on `D`. 2m
    Bit0D = 0x42,
    /// `BIT 0, E`: test if bit 0 is set on `E`. 2m
    Bit0E = 0x43,
    /// `BIT 0, H`: test if bit 0 is set on `H`. 2m
    Bit0H = 0x44,
    /// `BIT 0, L`: test if bit 0 is set on `L`. 2m
    Bit0L = 0x45,
    /// `BIT 0, (HL)`: test if bit 0 is set on `(HL)`. 4m
    Bit0HL = 0x46,
    /// `BIT 0, A`: test if bit 0 is set on `A`. 2m
    Bit0A = 0x47,
    /// `BIT 1, B`: test if bit 1 is set on `B`. 2m
    Bit1B = 0x48,
    /// `BIT 1, C`: test if bit 1 is set on `C`. 2m
    Bit1C = 0x49,
    /// `BIT 1, D`: test if bit 1 is set on `D`. 2m
    Bit1D = 0x4a,
    /// `BIT 1, E`: test if bit 1 is set on `E`. 2m
    Bit1E = 0x4b,
    /// `BIT 1, H`: test if bit 1 is set on `H`. 2m
    Bit1H = 0x4c,
    /// `BIT 1, L`: test if bit 1 is set on `L`. 2m
    Bit1L = 0x4d,
    /// `BIT 1, (HL)`: test if bit 1 is set on `(HL)`. 4m
    Bit1HL = 0x4e,
    /// `BIT 1, A`: test if bit 1 is set on `A`. 2m
    Bit1A = 0x4f,
    /// `BIT 2, B`: test if bit 2 is set on `B`. 2m
    Bit2B = 0x50,
    /// `BIT 2, C`: test if bit 2 is set on `C`. 2m
    Bit2C = 0x51,
    /// `BIT 2, D`: test if bit 2 is set on `D`. 2m
    Bit2D = 0x52,
    /// `BIT 2, E`: test if bit 2 is set on `E`. 2m
    Bit2E = 0x53,
    /// `BIT 2, H`: test if bit 2 is set on `H`. 2m
    Bit2H = 0x54,
    /// `BIT 2, L`: test if bit 2 is set on `L`. 2m
    Bit2L = 0x55,
    /// `BIT 2, (HL)`: test if bit 2 is set on `(HL)`. 4m
    Bit2HL = 0x56,
    /// `BIT 2, A`: test if bit 2 is set on `A`. 2m
    Bit2A = 0x57,
    /// `BIT 3, B`: test if bit 3 is set on `B`. 2m
    Bit3B = 0x58,
    /// `BIT 3, C`: test if bit 3 is set on `C`. 2m
    Bit3C = 0x59,
    /// `BIT 3, D`: test if bit 3 is set on `D`. 2m
    Bit3D = 0x5a,
    /// `BIT 3, E`: test if bit 3 is set on `E`. 2m
    Bit3E = 0x5b,
    /// `BIT 3, H`: test if bit 3 is set on `H`. 2m
    Bit3H = 0x5c,
    /// `BIT 3, L`: test if bit 3 is set on `L`. 2m
    Bit3L = 0x5d,
    /// `BIT 3, (HL)`: test if bit 3 is set on `(HL)`. 4m
    Bit3HL = 0x5e,
    /// `BIT 3, A`: test if bit 3 is set on `A`. 2m
    Bit3A = 0x5f,
    /// `BIT 4, B`: test if bit 4 is set on `B`. 2m
    Bit4B = 0x60,
    /// `BIT 4, C`: test if bit 4 is set on `C`. 2m
    Bit4C = 0x61,
    /// `BIT 4, D`: test if bit 4 is set on `D`. 2m
    Bit4D = 0x62,
    /// `BIT 4, E`: test if bit 4 is set on `E`. 2m
    Bit4E = 0x63,
    /// `BIT 4, H`: test if bit 4 is set on `H`. 2m
    Bit4H = 0x64,
    /// `BIT 4, L`: test if bit 4 is set on `L`. 2m
    Bit4L = 0x65,
    /// `BIT 4, (HL)`: test if bit 4 is set on `(HL)`. 4m
    Bit4HL = 0x66,
    /// `BIT 4, A`: test if bit 4 is set on `A`. 2m
    Bit4A = 0x67,
    /// `BIT 5, B`: test if bit 5 is set on `B`. 2m
    Bit5B = 0x68,
    /// `BIT 5, C`: test if bit 5 is set on `C`. 2m
    Bit5C = 0x69,
    /// `BIT 5, D`: test if bit 5 is set on `D`. 2m
    Bit5D = 0x6a,
    /// `BIT 5, E`: test if bit 5 is set on `E`. 2m
    Bit5E = 0x6b,
    /// `BIT 5, H`: test if bit 5 is set on `H`. 2m
    Bit5H = 0x6c,
    /// `BIT 5, L`: test if bit 5 is set on `L`. 2m
    Bit5L = 0x6d,
    /// `BIT 5, (HL)`: test if bit 5 is set on `(HL)`. 4m
    Bit5HL = 0x6e,
    /// `BIT 5, A`: test if bit 5 is set on `A`. 2m
    Bit5A = 0x6f,
    /// `BIT 6, B`: test if bit 6 is set on `B`. 2m
    Bit6B = 0x70,
    /// `BIT 6, C`: test if bit 6 is set on `C`. 2m
    Bit6C = 0x71,
    /// `BIT 6, D`: test if bit 6 is set on `D`. 2m
    Bit6D = 0x72,
    /// `BIT 6, E`: test if bit 6 is set on `E`. 2m
    Bit6E = 0x73,
    /// `BIT 6, H`: test if bit 6 is set on `H`. 2m
    Bit6H = 0x74,
    /// `BIT 6, L`: test if bit 6 is set on `L`. 2m
    Bit6L = 0x75,
    /// `BIT 6, (HL)`: test if bit 6 is set on `(HL)`. 4m
    Bit6HL = 0x76,
    /// `BIT 6, A`: test if bit 6 is set on `A`. 2m
    Bit6A = 0x77,
    /// `BIT 7, B`: test if bit 7 is set on `B`. 2m
    Bit7B = 0x78,
    /// `BIT 7, C`: test if bit 7 is set on `C`. 2m
    Bit7C = 0x79,
    /// `BIT 7, D`: test if bit 7 is set on `D`. 2m
    Bit7D = 0x7a,
    /// `BIT 7, E`: test if bit 7 is set on `E`. 2m
    Bit7E = 0x7b,
    /// `BIT 7, H`: test if bit 7 is set on `H`. 2m
    Bit7H = 0x7c,
    /// `BIT 7, L`: test if bit 7 is set on `L`. 2m
    Bit7L = 0x7d,
    /// `BIT 7, (HL)`: test if bit 7 is set on `(HL)`. 4m
    Bit7HL = 0x7e,
    /// `BIT 7, A`: test if bit 7 is set on `A`. 2m
    Bit7A = 0x7f,

    /// `RES 0, B`: set bit 0 to `0` on `B`. 2m
    Res0B = 0x80,
    /// `RES 0, C`: set bit 0 to `0` on `C`. 2m
    Res0C = 0x81,
    /// `RES 0, D`: set bit 0 to `0` on `D`. 2m
    Res0D = 0x82,
    /// `RES 0, E`: set bit 0 to `0` on `E`. 2m
    Res0E = 0x83,
    /// `RES 0, H`: set bit 0 to `0` on `H`. 2m
    Res0H = 0x84,
    /// `RES 0, L`: set bit 0 to `0` on `L`. 2m
    Res0L = 0x85,
    /// `RES 0, (HL)`: set bit 0 to `0` on `(HL)`. 2m
    Res0HL = 0x86,
    /// `RES 0, A`: set bit 0 to `0` on `A`. 2m
    Res0A = 0x87,
    /// `RES 1, B`: set bit 1 to `0` on `B`. 2m
    Res1B = 0x88,
    /// `RES 1, C`: set bit 1 to `0` on `C`. 2m
    Res1C = 0x89,
    /// `RES 1, D`: set bit 1 to `0` on `D`. 2m
    Res1D = 0x8a,
    /// `RES 1, E`: set bit 1 to `0` on `E`. 2m
    Res1E = 0x8b,
    /// `RES 1, H`: set bit 1 to `0` on `H`. 2m
    Res1H = 0x8c,
    /// `RES 1, L`: set bit 1 to `0` on `L`. 2m
    Res1L = 0x8d,
    /// `RES 1, (HL)`: set bit 1 to `0` on `(HL)`. 2m
    Res1HL = 0x8e,
    /// `RES 1, A`: set bit 1 to `0` on `A`. 2m
    Res1A = 0x8f,
    /// `RES 2, B`: set bit 2 to `0` on `B`. 2m
    Res2B = 0x90,
    /// `RES 2, C`: set bit 2 to `0` on `C`. 2m
    Res2C = 0x91,
    /// `RES 2, D`: set bit 2 to `0` on `D`. 2m
    Res2D = 0x92,
    /// `RES 2, E`: set bit 2 to `0` on `E`. 2m
    Res2E = 0x93,
    /// `RES 2, H`: set bit 2 to `0` on `H`. 2m
    Res2H = 0x94,
    /// `RES 2, L`: set bit 2 to `0` on `L`. 2m
    Res2L = 0x95,
    /// `RES 2, (HL)`: set bit 2 to `0` on `(HL)`. 2m
    Res2HL = 0x96,
    /// `RES 2, A`: set bit 2 to `0` on `A`. 2m
    Res2A = 0x97,
    /// `RES 3, B`: set bit 3 to `0` on `B`. 2m
    Res3B = 0x98,
    /// `RES 3, C`: set bit 3 to `0` on `C`. 2m
    Res3C = 0x99,
    /// `RES 3, D`: set bit 3 to `0` on `D`. 2m
    Res3D = 0x9a,
    /// `RES 3, E`: set bit 3 to `0` on `E`. 2m
    Res3E = 0x9b,
    /// `RES 3, H`: set bit 3 to `0` on `H`. 2m
    Res3H = 0x9c,
    /// `RES 3, L`: set bit 3 to `0` on `L`. 2m
    Res3L = 0x9d,
    /// `RES 3, (HL)`: set bit 3 to `0` on `(HL)`. 2m
    Res3HL = 0x9e,
    /// `RES 3, A`: set bit 3 to `0` on `A`. 2m
    Res3A = 0x9f,
    /// `RES 4, B`: set bit 4 to `0` on `B`. 2m
    Res4B = 0xa0,
    /// `RES 4, C`: set bit 4 to `0` on `C`. 2m
    Res4C = 0xa1,
    /// `RES 4, D`: set bit 4 to `0` on `D`. 2m
    Res4D = 0xa2,
    /// `RES 4, E`: set bit 4 to `0` on `E`. 2m
    Res4E = 0xa3,
    /// `RES 4, H`: set bit 4 to `0` on `H`. 2m
    Res4H = 0xa4,
    /// `RES 4, L`: set bit 4 to `0` on `L`. 2m
    Res4L = 0xa5,
    /// `RES 4, (HL)`: set bit 4 to `0` on `(HL)`. 2m
    Res4HL = 0xa6,
    /// `RES 4, A`: set bit 4 to `0` on `A`. 2m
    Res4A = 0xa7,
    /// `RES 5, B`: set bit 5 to `0` on `B`. 2m
    Res5B = 0xa8,
    /// `RES 5, C`: set bit 5 to `0` on `C`. 2m
    Res5C = 0xa9,
    /// `RES 5, D`: set bit 5 to `0` on `D`. 2m
    Res5D = 0xaa,
    /// `RES 5, E`: set bit 5 to `0` on `E`. 2m
    Res5E = 0xab,
    /// `RES 5, H`: set bit 5 to `0` on `H`. 2m
    Res5H = 0xac,
    /// `RES 5, L`: set bit 5 to `0` on `L`. 2m
    Res5L = 0xad,
    /// `RES 5, (HL)`: set bit 5 to `0` on `(HL)`. 2m
    Res5HL = 0xae,
    /// `RES 5, A`: set bit 5 to `0` on `A`. 2m
    Res5A = 0xaf,
    /// `RES 6, B`: set bit 6 to `0` on `B`. 2m
    Res6B = 0xb0,
    /// `RES 6, C`: set bit 6 to `0` on `C`. 2m
    Res6C = 0xb1,
    /// `RES 6, D`: set bit 6 to `0` on `D`. 2m
    Res6D = 0xb2,
    /// `RES 6, E`: set bit 6 to `0` on `E`. 2m
    Res6E = 0xb3,
    /// `RES 6, H`: set bit 6 to `0` on `H`. 2m
    Res6H = 0xb4,
    /// `RES 6, L`: set bit 6 to `0` on `L`. 2m
    Res6L = 0xb5,
    /// `RES 6, (HL)`: set bit 6 to `0` on `(HL)`. 2m
    Res6HL = 0xb6,
    /// `RES 6, A`: set bit 6 to `0` on `A`. 2m
    Res6A = 0xb7,
    /// `RES 7, B`: set bit 7 to `0` on `B`. 2m
    Res7B = 0xb8,
    /// `RES 7, C`: set bit 7 to `0` on `C`. 2m
    Res7C = 0xb9,
    /// `RES 7, D`: set bit 7 to `0` on `D`. 2m
    Res7D = 0xba,
    /// `RES 7, E`: set bit 7 to `0` on `E`. 2m
    Res7E = 0xbb,
    /// `RES 7, H`: set bit 7 to `0` on `H`. 2m
    Res7H = 0xbc,
    /// `RES 7, L`: set bit 7 to `0` on `L`. 2m
    Res7L = 0xbd,
    /// `RES 7, (HL)`: set bit 7 to `0` on `(HL)`. 2m
    Res7HL = 0xbe,
    /// `RES 7, A`: set bit 7 to `0` on `A`. 2m
    Res7A = 0xbf,

    /// `SET 0, B`: set bit 0 to `1` on `B`. 2m
    Set0B = 0xc0,
    /// `SET 0, C`: set bit 0 to `1` on `C`. 2m
    Set0C = 0xc1,
    /// `SET 0, D`: set bit 0 to `1` on `D`. 2m
    Set0D = 0xc2,
    /// `SET 0, E`: set bit 0 to `1` on `E`. 2m
    Set0E = 0xc3,
    /// `SET 0, H`: set bit 0 to `1` on `H`. 2m
    Set0H = 0xc4,
    /// `SET 0, L`: set bit 0 to `1` on `L`. 2m
    Set0L = 0xc5,
    /// `SET 0, (HL)`: set bit 0 to `1` on `(HL)`. 2m
    Set0HL = 0xc6,
    /// `SET 0, A`: set bit 0 to `1` on `A`. 2m
    Set0A = 0xc7,
    /// `SET 1, B`: set bit 1 to `1` on `B`. 2m
    Set1B = 0xc8,
    /// `SET 1, C`: set bit 1 to `1` on `C`. 2m
    Set1C = 0xc9,
    /// `SET 1, D`: set bit 1 to `1` on `D`. 2m
    Set1D = 0xca,
    /// `SET 1, E`: set bit 1 to `1` on `E`. 2m
    Set1E = 0xcb,
    /// `SET 1, H`: set bit 1 to `1` on `H`. 2m
    Set1H = 0xcc,
    /// `SET 1, L`: set bit 1 to `1` on `L`. 2m
    Set1L = 0xcd,
    /// `SET 1, (HL)`: set bit 1 to `1` on `(HL)`. 2m
    Set1HL = 0xce,
    /// `SET 1, A`: set bit 1 to `1` on `A`. 2m
    Set1A = 0xcf,
    /// `SET 2, B`: set bit 2 to `1` on `B`. 2m
    Set2B = 0xd0,
    /// `SET 2, C`: set bit 2 to `1` on `C`. 2m
    Set2C = 0xd1,
    /// `SET 2, D`: set bit 2 to `1` on `D`. 2m
    Set2D = 0xd2,
    /// `SET 2, E`: set bit 2 to `1` on `E`. 2m
    Set2E = 0xd3,
    /// `SET 2, H`: set bit 2 to `1` on `H`. 2m
    Set2H = 0xd4,
    /// `SET 2, L`: set bit 2 to `1` on `L`. 2m
    Set2L = 0xd5,
    /// `SET 2, (HL)`: set bit 2 to `1` on `(HL)`. 2m
    Set2HL = 0xd6,
    /// `SET 2, A`: set bit 2 to `1` on `A`. 2m
    Set2A = 0xd7,
    /// `SET 3, B`: set bit 3 to `1` on `B`. 2m
    Set3B = 0xd8,
    /// `SET 3, C`: set bit 3 to `1` on `C`. 2m
    Set3C = 0xd9,
    /// `SET 3, D`: set bit 3 to `1` on `D`. 2m
    Set3D = 0xda,
    /// `SET 3, E`: set bit 3 to `1` on `E`. 2m
    Set3E = 0xdb,
    /// `SET 3, H`: set bit 3 to `1` on `H`. 2m
    Set3H = 0xdc,
    /// `SET 3, L`: set bit 3 to `1` on `L`. 2m
    Set3L = 0xdd,
    /// `SET 3, (HL)`: set bit 3 to `1` on `(HL)`. 2m
    Set3HL = 0xde,
    /// `SET 3, A`: set bit 3 to `1` on `A`. 2m
    Set3A = 0xdf,
    /// `SET 4, B`: set bit 4 to `1` on `B`. 2m
    Set4B = 0xe0,
    /// `SET 4, C`: set bit 4 to `1` on `C`. 2m
    Set4C = 0xe1,
    /// `SET 4, D`: set bit 4 to `1` on `D`. 2m
    Set4D = 0xe2,
    /// `SET 4, E`: set bit 4 to `1` on `E`. 2m
    Set4E = 0xe3,
    /// `SET 4, H`: set bit 4 to `1` on `H`. 2m
    Set4H = 0xe4,
    /// `SET 4, L`: set bit 4 to `1` on `L`. 2m
    Set4L = 0xe5,
    /// `SET 4, (HL)`: set bit 4 to `1` on `(HL)`. 2m
    Set4HL = 0xe6,
    /// `SET 4, A`: set bit 4 to `1` on `A`. 2m
    Set4A = 0xe7,
    /// `SET 5, B`: set bit 5 to `1` on `B`. 2m
    Set5B = 0xe8,
    /// `SET 5, C`: set bit 5 to `1` on `C`. 2m
    Set5C = 0xe9,
    /// `SET 5, D`: set bit 5 to `1` on `D`. 2m
    Set5D = 0xea,
    /// `SET 5, E`: set bit 5 to `1` on `E`. 2m
    Set5E = 0xeb,
    /// `SET 5, H`: set bit 5 to `1` on `H`. 2m
    Set5H = 0xec,
    /// `SET 5, L`: set bit 5 to `1` on `L`. 2m
    Set5L = 0xed,
    /// `SET 5, (HL)`: set bit 5 to `1` on `(HL)`. 2m
    Set5HL = 0xee,
    /// `SET 5, A`: set bit 5 to `1` on `A`. 2m
    Set5A = 0xef,
    /// `SET 6, B`: set bit 6 to `1` on `B`. 2m
    Set6B = 0xf0,
    /// `SET 6, C`: set bit 6 to `1` on `C`. 2m
    Set6C = 0xf1,
    /// `SET 6, D`: set bit 6 to `1` on `D`. 2m
    Set6D = 0xf2,
    /// `SET 6, E`: set bit 6 to `1` on `E`. 2m
    Set6E = 0xf3,
    /// `SET 6, H`: set bit 6 to `1` on `H`. 2m
    Set6H = 0xf4,
    /// `SET 6, L`: set bit 6 to `1` on `L`. 2m
    Set6L = 0xf5,
    /// `SET 6, (HL)`: set bit 6 to `1` on `(HL)`. 2m
    Set6HL = 0xf6,
    /// `SET 6, A`: set bit 6 to `1` on `A`. 2m
    Set6A = 0xf7,
    /// `SET 7, B`: set bit 7 to `1` on `B`. 2m
    Set7B = 0xf8,
    /// `SET 7, C`: set bit 7 to `1` on `C`. 2m
    Set7C = 0xf9,
    /// `SET 7, D`: set bit 7 to `1` on `D`. 2m
    Set7D = 0xfa,
    /// `SET 7, E`: set bit 7 to `1` on `E`. 2m
    Set7E = 0xfb,
    /// `SET 7, H`: set bit 7 to `1` on `H`. 2m
    Set7H = 0xfc,
    /// `SET 7, L`: set bit 7 to `1` on `L`. 2m
    Set7L = 0xfd,
    /// `SET 7, (HL)`: set bit 7 to `1` on `(HL)`. 2m
    Set7HL = 0xfe,
    /// `SET 7, A`: set bit 7 to `1` on `A`. 2m
    Set7A = 0xff,
}
