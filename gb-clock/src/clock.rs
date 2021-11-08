use crate::{ticker::cycle, Ticker};
use gb_bus::Bus;
use std::marker::PhantomData;
use std::ops::DerefMut;

/// Ensure that the various process unit execute their instructions in the right order.
pub struct Clock<B: Bus<u8> + Bus<u16>> {
    curr_frame_cycle: usize,
    phantom_bus: PhantomData<B>,
}

impl<B: Bus<u8> + Bus<u16>> Clock<B> {
    /// The amount of cycles to execute per frame.
    pub const CYCLES_PER_FRAME: usize = 17556;

    /// A single clock cycle, during which each [Ticker] will tick 1 or 4 times depending on their [Tick](crate::Tick) type.
    ///
    /// Its return value indicate if the current frame is incomplete.
    pub fn cycle<
        CPU: DerefMut<Target = impl Ticker>,
        PPU: DerefMut<Target = impl Ticker>,
        TIMER: DerefMut<Target = impl Ticker>,
    >(
        &mut self,
        addr_bus: &mut B,
        cpu: CPU,
        ppu: PPU,
        timer: TIMER,
    ) -> bool {
        cycle(timer, addr_bus);
        cycle(cpu, addr_bus);
        cycle(ppu, addr_bus);
        self.curr_frame_cycle += 1;
        !self.frame_ready()
    }

    /// Indicate if the current frame has been completed or not.
    pub fn frame_ready(&mut self) -> bool {
        self.curr_frame_cycle %= Self::CYCLES_PER_FRAME;
        self.curr_frame_cycle == 0
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
