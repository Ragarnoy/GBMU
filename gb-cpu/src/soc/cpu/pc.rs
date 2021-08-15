use crate::memory::Memory;
use crate::error::Error;
use crate::bus::Bus;

//If successfull, the next function returns the byte at PC address and advances PC by one

pub type Pc = u16;

impl NextPc for Pc {
     fn next(&mut self, memory:&Memory) -> Result<u8, Error>{
        match memory.get(*self) {
            Ok(byte) => {
                *self = self.wrapping_add(1);
                Ok(byte)
            }
            Err(_) => Err(Error::InvalidPC(*self))
        }
    }
}


pub trait NextPc {
     fn next(&mut self, memory:&Memory) -> Result<u8, Error>;
}
