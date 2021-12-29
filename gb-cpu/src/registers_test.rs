mod read {
    use crate::{
        interfaces::{Read8BitsReg, Read8BitsRegExt},
        registers::Registers,
    };

    #[test]
    fn af() {
        let regs = Registers {
            af: 0xaaff,
            ..Registers::default()
        };

        assert_eq!(regs.a(), 0xaa);
        assert_eq!(regs.f(), 0xff);
    }

    #[test]
    fn bc() {
        let regs = Registers {
            bc: 0xbbcc,
            ..Registers::default()
        };

        assert_eq!(regs.b(), 0xbb);
        assert_eq!(regs.c(), 0xcc);
    }

    #[test]
    fn de() {
        let regs = Registers {
            de: 0xddee,
            ..Registers::default()
        };

        assert_eq!(regs.d(), 0xdd);
        assert_eq!(regs.e(), 0xee);
    }

    #[test]
    fn hl() {
        let regs = Registers {
            hl: 0x8833,
            ..Registers::default()
        };

        assert_eq!(regs.h(), 0x88);
        assert_eq!(regs.l(), 0x33);
    }
}

mod write {
    use crate::{
        interfaces::{Read8BitsReg, Read8BitsRegExt, Write8BitsReg, Write8BitsRegExt},
        registers::Registers,
    };

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
fn read_flag() {
    use crate::{
        interfaces::{ReadFlagReg, Write8BitsRegExt},
        registers::{Registers, CARRY_MASK, HALF_CARRY_MASK, SUBSTRACTION_MASK, ZERO_MASK},
    };

    let mut regs = Registers::default();
    regs.set_f((ZERO_MASK | SUBSTRACTION_MASK | HALF_CARRY_MASK | CARRY_MASK) as u8);

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

mod write_flag {
    use crate::{
        interfaces::{ReadFlagReg, WriteFlagReg},
        registers::Registers,
    };

    #[test]
    fn zero() {
        let mut regs = Registers::default();

        assert!(!regs.zero());

        regs.set_zero(true);
        assert!(regs.zero());

        regs.set_zero(false);
        assert!(!regs.zero());
    }

    #[test]
    fn subtraction() {
        let mut regs = Registers::default();

        assert!(!regs.subtraction());

        regs.set_subtraction(true);
        assert!(regs.subtraction());

        regs.set_subtraction(false);
        assert!(!regs.subtraction());
    }

    #[test]
    fn half_carry() {
        let mut regs = Registers::default();

        assert!(!regs.half_carry());

        regs.set_half_carry(true);
        assert!(regs.half_carry());

        regs.set_half_carry(false);
        assert!(!regs.half_carry());
    }

    #[test]
    fn carry() {
        let mut regs = Registers::default();

        assert!(!regs.carry());

        regs.set_carry(true);
        assert!(regs.carry());

        regs.set_carry(false);
        assert!(!regs.carry());
    }
}
