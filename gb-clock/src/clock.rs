use crate::{ticker::cycle, Debuger, Ticker};
use gb_bus::Bus;
use std::marker::PhantomData;

/// Ensure that the various process unit execute their instructions in the right order.
pub struct Clock<B: Bus<u8> + Bus<u16>> {
    curr_frame_cycle: usize,
    phantom_bus: PhantomData<B>,
}

impl<B: Bus<u8> + Bus<u16>> Clock<B> {
    /// The amount of cycles to execute per frame.
    pub const CYCLES_PER_FRAME: usize = 17556;

    /// A single clock cycle, during which each [Ticker] will tick 1 or 4 times depending on their [Tick](crate::Tick) type.
    pub fn cycle<CPU: Ticker, PPU: Ticker>(
        &mut self,
        adr_bus: &mut B,
        cpu: &mut CPU,
        ppu: &mut PPU,
    ) {
        cycle(cpu, adr_bus);
        cycle(ppu, adr_bus);
        self.curr_frame_cycle += 1;
    }

    /// Indicate if the current frame has been completed or not.
    pub fn frame_ready(&mut self) -> bool {
        self.curr_frame_cycle %= Self::CYCLES_PER_FRAME;
        self.curr_frame_cycle == 0
    }

    /// Execute enough cycles to complete the current frame.
    ///
    /// if a [Debuger] is given, it will check breakpoints after each clock cycle and interrupt the execution if needed.
    pub fn frame<CPU: Ticker, PPU: Ticker>(
        &mut self,
        adr_bus: &mut B,
        dbg: Option<&dyn Debuger<B>>,
        cpu: &mut CPU,
        ppu: &mut PPU,
    ) {
        self.curr_frame_cycle %= Self::CYCLES_PER_FRAME;
        match dbg {
            Some(dbg) => {
                while self.curr_frame_cycle < Self::CYCLES_PER_FRAME {
                    self.cycle(adr_bus, cpu, ppu);
                    if dbg.breakpoints(adr_bus) {
                        return;
                    }
                }
            }
            None => {
                while self.curr_frame_cycle < Self::CYCLES_PER_FRAME {
                    self.cycle(adr_bus, cpu, ppu);
                }
            }
        }
        self.curr_frame_cycle = 0;
    }
}

impl<B: Bus<u8> + Bus<u16>> Default for Clock<B> {
    fn default() -> Self {
        Self {
            curr_frame_cycle: 0,
            phantom_bus: PhantomData,
        }
    }
}
