use crate::Timer;
use gb_bus::{Bus, Lock};
use gb_clock::Ticker;
use gb_test::MockBus;

macro_rules! test_mode {
    ($name:ident, $tac:literal, $step:literal) => {
        #[test]
        fn $name() {
            let mut fake_bus = MockBus::default();
            let mut timer = Timer::default();
            timer.tac = 0b100 | $tac;
            timer.tima = 0xff;
            for i in 0..$step - 1 {
                timer.tick(&mut fake_bus);
                let int_flag: u8 = fake_bus
                    .read(0xff0f, Some(Lock::Debugger))
                    .unwrap_or_default();
                assert_eq!(
                    int_flag, 0,
                    "failed step {:04x}/{:04x}: expected 0, got {:#04b} ({:x?})",
                    i, $step, int_flag, timer
                );
            }
            timer.tick(&mut fake_bus);
            let int_flag: u8 = fake_bus
                .read(0xff0f, Some(Lock::Debugger))
                .unwrap_or_default();
            assert!(
                int_flag & Timer::TIMER_INT_MASK != 0,
                "expected mask {:#b} but got {:#b} ({:x?})",
                Timer::TIMER_INT_MASK,
                int_flag,
                timer
            );
        }
    };
}

test_mode!(test_mode_0, 0, 1024);
test_mode!(test_mode_1, 1, 16);
test_mode!(test_mode_2, 2, 64);
test_mode!(test_mode_3, 3, 256);
