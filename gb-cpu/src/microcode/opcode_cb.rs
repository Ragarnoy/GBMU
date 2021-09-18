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
}
