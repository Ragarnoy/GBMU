use gb_cpu::address_bus::{Error, RomOperation};

pub struct RomOnlyController {
    rom: [u8; 32_768],
}

impl RomOperation for RomOnlyController {
    fn read_rom(&self, addr: Position) -> Result<(), Error> {
        if addr.relative < self.rom.len() {
            Ok(self.rom[addr.relative])
        } else {
            Err(Error::BusError(addr.absolute))
        }
    }
}
