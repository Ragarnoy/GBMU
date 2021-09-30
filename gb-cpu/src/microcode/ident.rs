use super::{MicrocodeController, State};
use crate::interfaces::Read8BitsReg;
use std::convert::From;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reg16 {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ident {
    Reg8(Reg8),
    Reg16(Reg16),
    IndirectHL8,
    /// u8 argument, need to be retrieve from cache
    Raw8,
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

pub fn get_u8_from_ident(ident: Ident, state: &mut State, ctl: &mut MicrocodeController) -> u8 {
    match ident {
        Ident::Reg8(r8) => match r8 {
            Reg8::A => state.regs.a(),
            Reg8::B => state.regs.b(),
            Reg8::C => state.regs.c(),
            Reg8::D => state.regs.d(),
            Reg8::E => state.regs.e(),
            Reg8::H => state.regs.h(),
            Reg8::L => state.regs.l(),
        },
        Ident::Raw8 | Ident::IndirectHL8 => ctl.pop(),
        _ => panic!("cannot get an u8 with type {:?}", ident),
    }
}
