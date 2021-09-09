use crate::interfaces::{
    Read8BitsReg, Read8BitsRegExt, ReadFlagReg, Write8BitsReg, Write8BitsRegExt, WriteFlagReg,
};

#[derive(Default)]
pub struct Registers {
    /// Accumulator & Flags
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    /// Stack Pointer
    pub sp: u16,
    /// Program Counter / Pointer
    pub pc: u16,
}

impl Read8BitsReg for Registers {
    fn a(&self) -> u8 {
        self.af.to_be_bytes()[1]
    }

    fn b(&self) -> u8 {
        self.bc.to_be_bytes()[1]
    }

    fn c(&self) -> u8 {
        self.bc.to_be_bytes()[0]
    }

    fn d(&self) -> u8 {
        self.de.to_be_bytes()[1]
    }

    fn e(&self) -> u8 {
        self.de.to_be_bytes()[0]
    }

    fn h(&self) -> u8 {
        self.hl.to_be_bytes()[1]
    }

    fn l(&self) -> u8 {
        self.hl.to_be_bytes()[0]
    }
}

impl Read8BitsRegExt for Registers {
    fn f(&self) -> u8 {
        self.af.to_be_bytes()[0]
    }
}

impl Write8BitsReg for Registers {
    fn set_a(&mut self, value: u8) {
        self.af = u16::from_be_bytes([self.f(), value]);
    }

    fn set_b(&mut self, value: u8) {
        self.bc = u16::from_be_bytes([self.c(), value]);
    }

    fn set_c(&mut self, value: u8) {
        self.bc = u16::from_be_bytes([value, self.b()]);
    }

    fn set_d(&mut self, value: u8) {
        self.de = u16::from_be_bytes([self.e(), value]);
    }

    fn set_e(&mut self, value: u8) {
        self.de = u16::from_be_bytes([value, self.d()]);
    }

    fn set_h(&mut self, value: u8) {
        self.hl = u16::from_be_bytes([self.l(), value]);
    }

    fn set_l(&mut self, value: u8) {
        self.hl = u16::from_be_bytes([value, self.h()]);
    }
}

impl Write8BitsRegExt for Registers {
    fn set_f(&mut self, value: u8) {
        self.af = u16::from_be_bytes([value, self.a()]);
    }
}

impl ReadFlagReg for Registers {
    fn zero(&self) -> bool {
        self.f() >> 7 == 0b1
    }

    fn subtraction(&self) -> bool {
        (self.f() >> 6 & 1) == 1
    }

    fn half_carry(&self) -> bool {
        (self.f() >> 5 & 1) == 1
    }

    fn carry(&self) -> bool {
        (self.f() >> 4 & 1) == 1
    }

    fn raw(&self) -> u8 {
        self.f()
    }
}

const ZERO_MASK: u16 = 0b1000_0000;
const SUBTRACTION_MASK: u16 = 0b100_0000;
const HALF_CARRY_MASK: u16 = 0b10_0000;
const CARRY_MASK: u16 = 0b1_0000;

impl WriteFlagReg for Registers {
    fn set_zero(&mut self, value: bool) {
        if value {
            self.af |= ZERO_MASK << 8;
        } else {
            self.af &= !ZERO_MASK << 8;
        }
    }

    fn set_subtraction(&mut self, value: bool) {
        if value {
            self.af |= SUBTRACTION_MASK << 8;
        } else {
            self.af &= !SUBTRACTION_MASK << 8;
        }
    }

    fn set_half_carry(&mut self, value: bool) {
        if value {
            self.af |= HALF_CARRY_MASK << 8;
        } else {
            self.af &= !HALF_CARRY_MASK << 8;
        }
    }

    fn set_carry(&mut self, value: bool) {
        if value {
            self.af |= CARRY_MASK << 8;
        } else {
            self.af &= !CARRY_MASK << 8;
        }
    }

    fn raw(&mut self, value: u8) {
        self.set_f(value)
    }
}

#[cfg(test)]
mod test_read {
    use super::Registers;
    use crate::interfaces::{Read8BitsReg, Read8BitsRegExt};

    #[test]
    fn af() {
        let regs = Registers {
            af: u16::to_be(0xaaff),
            ..Registers::default()
        };

        assert_eq!(regs.a(), 0xaa);
        assert_eq!(regs.f(), 0xff);
    }

    #[test]
    fn bc() {
        let regs = Registers {
            bc: u16::to_be(0xbbcc),
            ..Registers::default()
        };

        assert_eq!(regs.b(), 0xbb);
        assert_eq!(regs.c(), 0xcc);
    }

    #[test]
    fn de() {
        let regs = Registers {
            de: u16::to_be(0xddee),
            ..Registers::default()
        };

        assert_eq!(regs.d(), 0xdd);
        assert_eq!(regs.e(), 0xee);
    }

    #[test]
    fn hl() {
        let regs = Registers {
            hl: u16::to_be(0x8833),
            ..Registers::default()
        };

        assert_eq!(regs.h(), 0x88);
        assert_eq!(regs.l(), 0x33);
    }
}

#[cfg(test)]
mod test_write {
    use super::Registers;
    use crate::interfaces::{Read8BitsReg, Read8BitsRegExt, Write8BitsReg, Write8BitsRegExt};

    #[test]
    fn af() {
        let mut regs = Registers::default();

        assert_eq!(regs.a(), 0);
        assert_eq!(regs.f(), 0);

        regs.set_a(0xaa);
        regs.set_f(0xff);

        assert_eq!(regs.a(), 0xaa);
        assert_eq!(regs.f(), 0xff);
    }

    #[test]
    fn bc() {
        let mut regs = Registers::default();

        assert_eq!(regs.b(), 0);
        assert_eq!(regs.c(), 0);

        regs.set_b(0xbb);
        regs.set_c(0xcc);

        assert_eq!(regs.b(), 0xbb);
        assert_eq!(regs.c(), 0xcc);
    }

    #[test]
    fn de() {
        let mut regs = Registers::default();

        assert_eq!(regs.d(), 0);
        assert_eq!(regs.e(), 0);

        regs.set_d(0xdd);
        regs.set_e(0xee);

        assert_eq!(regs.d(), 0xdd);
        assert_eq!(regs.e(), 0xee);
    }

    #[test]
    fn hl() {
        let mut regs = Registers::default();

        assert_eq!(regs.h(), 0);
        assert_eq!(regs.l(), 0);

        regs.set_h(0x88);
        regs.set_l(0x33);

        assert_eq!(regs.h(), 0x88);
        assert_eq!(regs.l(), 0x33);
    }
}

#[test]
fn test_read_flag() {
    let mut regs = Registers::default();
    regs.set_f((ZERO_MASK | SUBTRACTION_MASK | HALF_CARRY_MASK | CARRY_MASK) as u8);

    assert!(regs.zero(), "ZERO flag should be set");
    assert!(regs.subtraction(), "SUBTRACTION flag should be set");
    assert!(regs.half_carry(), "HALF_CARRY flag should be set");
    assert!(regs.carry(), "CARRY flag should be set");

    regs.set_f(0);

    assert!(!regs.zero());
    assert!(!regs.subtraction());
    assert!(!regs.half_carry());
    assert!(!regs.carry());
}

#[cfg(test)]
mod test_write_flag {
    use super::Registers;
    use crate::interfaces::{ReadFlagReg, WriteFlagReg};

    #[test]
    fn zero() {
        let mut regs = Registers::default();

        assert!(!regs.zero());

        regs.set_zero(true);
        assert_eq!(regs.zero(), true);

        regs.set_zero(false);
        assert_eq!(regs.zero(), false);
    }

    #[test]
    fn subtraction() {
        let mut regs = Registers::default();

        assert!(!regs.subtraction());

        regs.set_subtraction(true);
        assert_eq!(regs.subtraction(), true);

        regs.set_subtraction(false);
        assert_eq!(regs.subtraction(), false);
    }

    #[test]
    fn half_carry() {
        let mut regs = Registers::default();

        assert!(!regs.half_carry());

        regs.set_half_carry(true);
        assert_eq!(regs.half_carry(), true);

        regs.set_half_carry(false);
        assert_eq!(regs.half_carry(), false);
    }

    #[test]
    fn carry() {
        let mut regs = Registers::default();

        assert!(!regs.carry());

        regs.set_carry(true);
        assert_eq!(regs.carry(), true);

        regs.set_carry(false);
        assert_eq!(regs.carry(), false);
    }
}
