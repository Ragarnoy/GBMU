use crate::Ticker;
use gb_bus::Bus;
use std::marker::PhantomData;

#[derive(Default)]
pub struct Clock<B: Bus<u8> + Bus<u16>> {
    phantom_bus: PhantomData<B>,
}

impl<B: Bus<u8> + Bus<u16>> Clock<B> {
    pub fn cycle(&self, adr_bus: &mut B, process_units: &mut Vec<&mut dyn Ticker<B>>) {
        for ticker in process_units {
            for _ in 0..ticker.cycle_count().into() {
                ticker.tick(adr_bus);
            }
        }
    }
}
