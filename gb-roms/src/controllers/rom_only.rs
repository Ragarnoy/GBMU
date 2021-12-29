use super::ROM_AREA_SIZE;
use gb_bus::{Address, Area, Error, FileOperation};
use std::io::{self, ErrorKind, Read};

pub struct RomOnlyController {
    rom: [u8; ROM_AREA_SIZE],
}

impl Default for RomOnlyController {
    fn default() -> Self {
        Self {
            rom: [0_u8; ROM_AREA_SIZE],
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

impl<A> FileOperation<A, Area> for RomOnlyController
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, addr: A) -> Result<u8, Error> {
        let address = addr.get_address();
        if address < self.rom.len() {
            Ok(self.rom[address])
        } else {
            Err(Error::bus_error(addr.into()))
        }
    }
}

#[test]
fn test_romonly_impl() {
    use gb_bus::{address::Addr, area::Area};

    let rom = RomOnlyController {
        rom: [42; ROM_AREA_SIZE],
    };

    assert_eq!(rom.read(Addr::from_offset(Area::Rom, 0x7fff, 0)), Ok(42));
}
