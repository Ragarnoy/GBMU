use gb_cpu::address_bus::{Error, Position, RomOperation};
use std::io::{self, ErrorKind, Read};

pub const MAX_ROM_ONLY_SIZE: usize = 32_768;

pub struct RomOnlyController {
    rom: [u8; MAX_ROM_ONLY_SIZE],
}

impl Default for RomOnlyController {
    fn default() -> Self {
        Self {
            rom: [0_u8; MAX_ROM_ONLY_SIZE],
        }
    }
}

impl RomOnlyController {
    pub fn from_file(mut file: impl Read) -> Result<Self, io::Error> {
        let mut ctl = RomOnlyController::default();

        if let Err(e) = file.read_exact(&mut ctl.rom) {
            if e.kind() != ErrorKind::UnexpectedEof {
                return Err(e);
            }
        }
        Ok(ctl)
    }
}

impl RomOperation for RomOnlyController {
    fn read_rom(&self, addr: Position) -> Result<u8, Error> {
        if (addr.relative as usize) < self.rom.len() {
            Ok(self.rom[addr.relative as usize])
        } else {
            Err(Error::BusError(addr.absolute))
        }
    }
}

#[test]
fn test_romonly_impl() {
    let rom = RomOnlyController {
        rom: [42; MAX_ROM_ONLY_SIZE],
    };

    assert_eq!(rom.read_rom(Position::from_offset(0x7fff, 0)), Ok(42));
}
