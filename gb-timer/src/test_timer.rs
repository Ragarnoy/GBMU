use crate::Timer;
use gb_bus::{Bus, Lock};
use gb_clock::Ticker;
use gb_test::MockBus;

macro_rules! test_tima {
    ($name:ident, $tac:literal, $step:literal) => {
        #[test]
        fn $name() {
            const INC_INTERVAL: u16 = $step;
            assert_eq!(INC_INTERVAL % Timer::INC_PER_TICK, 0);
            const STEPS: u16 = INC_INTERVAL / Timer::INC_PER_TICK;

            let mut fake_bus = MockBus::default();
            let mut timer = Timer::default();

            timer.tac = 0b100 | $tac;
            timer.tima = 0xff;

            for i in 0..STEPS {
                let int_flag: u8 = fake_bus
                    .read(0xff0f, Some(Lock::Debugger))
                    .unwrap_or_default();
                assert_eq!(
                    int_flag, 0,
                    "failed step {:04x}/{:04x}: expected 0, got {:#04b} ({:x?})",
                    i, $step, int_flag, timer
                );
                timer.tick(&mut fake_bus);
            }
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

test_tima!(test_mode_0, 0, 4096);
test_tima!(test_mode_1, 1, 16);
test_tima!(test_mode_2, 2, 64);
test_tima!(test_mode_3, 3, 256);

#[test]
fn test_div() {
    const DIV_INC_INTERVAL: u16 = 256;
    let mut fake_bus = MockBus::default();
    let mut timer = Timer::default();

    assert_eq!(DIV_INC_INTERVAL % Timer::INC_PER_TICK, 0);
    const STEPS: u16 = DIV_INC_INTERVAL / Timer::INC_PER_TICK;
    for i in 0..STEPS {
        assert_eq!(
            timer.div(),
            0,
            "div incremented too early at step {:4x}/{:4x}, ({:x?})",
            i,
            STEPS,
            timer
        );
        timer.tick(&mut fake_bus);
    }
    assert_eq!(
        timer.div(),
        1,
        "div was not incremented after the last tick ({:x?})",
        timer
    );
}
