use crate::AddressBus;

pub struct Iter<'a> {
    current_address: u16,
    stop: bool,
    bus: &'a AddressBus,
}

impl<'a> Iter<'a> {
    pub fn new(bus: &'a AddressBus) -> Self {
        Self {
            current_address: 0,
            stop: false,
            bus,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            let bit = self.bus.read(self.current_address).ok();
            match self.current_address {
                0xfea0..=0xfeff => self.current_address = 0xff00,
                0xffff => self.stop = true,
                _ => self.current_address += 1,
            }
            bit
        } else {
            None
        }
    }
}
