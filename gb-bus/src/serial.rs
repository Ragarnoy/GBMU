use crate::{Address, Error, FileOperation, IORegArea};

#[derive(Clone, Default)]
pub struct Serial {
    payload: u8,
    control: u8,
    buffer: String,
}

impl Serial {
    #[cfg(feature = "cgb")]
    const SC_MASK: u8 = 0x7c;
    #[cfg(not(feature = "cgb"))]
    const SC_MASK: u8 = 0x7e;

    const TRANSFER_FIELD: u8 = 0x80;
    const CLOCK_FIELD: u8 = 0x1;

    fn internal_data_to_transfer(&self) -> bool {
        self.control & (Serial::TRANSFER_FIELD | Serial::CLOCK_FIELD)
            == (Serial::TRANSFER_FIELD | Serial::CLOCK_FIELD)
    }

    fn transfer_finished(&mut self) {
        self.control &= !Serial::TRANSFER_FIELD;
    }

    fn update(&mut self) {
        if self.internal_data_to_transfer() {
            let ch = self.payload as char;
            log::debug!("Serial: {0:#02x}({1})", self.payload, ch.escape_default());
            if ch == '\n' {
                println!("{}", self.buffer.escape_default());
                self.buffer.clear();
            } else {
                self.buffer.push(ch);
            }
            self.transfer_finished()
        }
    }
}

impl Drop for Serial {
    fn drop(&mut self) {
        if !self.buffer.is_empty() {
            println!("{}", self.buffer);
        }
    }
}

impl<A> FileOperation<A, IORegArea> for Serial
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, addr: A) -> Result<u8, Error> {
        match addr.area_type() {
            IORegArea::SB => Ok(self.payload),
            IORegArea::SC => Ok(self.control | Serial::SC_MASK),
            _ => Err(Error::bus_error(addr.into())),
        }
    }

    fn write(&mut self, v: u8, addr: A) -> Result<(), Error> {
        match addr.area_type() {
            IORegArea::SB => self.payload = v,
            IORegArea::SC => {
                self.control = v & !Serial::SC_MASK;
                self.update();
            }
            _ => return Err(Error::bus_error(addr.into())),
        }
        Ok(())
    }
}
