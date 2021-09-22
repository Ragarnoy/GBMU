use std::convert::From;

#[derive(PartialEq, Eq)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(PartialEq, Eq)]
pub enum Reg16 {
    BC,
    DE,
    HL,
    SP,
}

#[derive(PartialEq, Eq)]
pub enum Ident {
    Reg8(Reg8),
    Reg16(Reg16),
    IndirectHL8,
}

impl From<Reg8> for Ident {
    fn from(r8: Reg8) -> Self {
        Self::Reg8(r8)
    }
}

impl From<Reg16> for Ident {
    fn from(r16: Reg16) -> Self {
        Self::Reg16(r16)
    }
}
