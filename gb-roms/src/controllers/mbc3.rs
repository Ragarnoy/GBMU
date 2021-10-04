use super::Controller;
use crate::header::Header;
use gb_bus::{Address, Area, Error, FileOperation};
use gb_rtc::{Naive, ReadRtcRegisters};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

type RamBank = [u8; MBC3::RAM_BANK_SIZE];
type RomBank = [u8; MBC3::ROM_BANK_SIZE];

pub struct MBC3 {
    rom_banks: Vec<RomBank>,
    ram_banks: Vec<RamBank>,
    regs: MBC3Regs,
    clock: Option<Naive>,
}

impl MBC3 {
    pub const ROM_BANK_SIZE: usize = 0x4000;
    pub const RAM_BANK_SIZE: usize = 0x2000;

    pub fn from_reader(mut reader: impl Read, header: Header) -> Result<Self, io::Error> {
        let mut ctl = MBC3::empty(header);

        for e in ctl.rom_banks.iter_mut() {
            reader.read_exact(e)?;
        }
        Ok(ctl)
    }

    pub fn empty(header: Header) -> Self {
        use crate::header::cartridge_type::CartridgeType::{
            Mbc3TimerBattery, Mbc3TimerRamBattery2,
        };

        let ram_amount = header.ram_size.get_bank_amounts();
        let rom_amount = header.rom_size.get_bank_amounts();
        let clock = match header.cartridge_type {
            Mbc3TimerBattery | Mbc3TimerRamBattery2 => Some(Naive::default()),
            _ => None,
        };
        Self {
            ram_banks: vec![[0_u8; MBC3::RAM_BANK_SIZE]; ram_amount],
            rom_banks: vec![[0_u8; MBC3::ROM_BANK_SIZE]; rom_amount],
            regs: MBC3Regs::default(),
            clock,
        }
    }

    fn read_rom(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        let address = addr.get_address();
        match address {
            0x0000..=0x3FFF => Ok(self.rom_banks[0][address]),
            0x4000..=0x7FFF => Ok(self.get_selected_rom_bank()[address]),
            _ => Err(Error::new_segfault(addr)),
        }
    }

    fn get_selected_rom_bank(&self) -> &RomBank {
        &self.rom_banks[self.regs.rom_bank as usize]
    }

    fn write_rom(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        let address = addr.get_address();
        match address {
            0x0000..=0x1FFF => self.regs.ram_enabled = (v & 0xF) == 0xA,
            0x2000..=0x3FFF => self.regs.rom_bank = if v == 0 { 1 } else { v & 0x7F },
            0x4000..=0x5FFF => self.regs.ram_bank = v & 0xC,
            0x6000..=0x7FFF => {
                if self.regs.last_writed_byte == Some(0_u8) && v == 1 {
                    self.latch_clock_data(addr)?;
                } else {
                    self.regs.last_writed_byte = Some(v);
                }
            }
            _ => return Err(Error::new_segfault(addr)),
        }
        Ok(())
    }

    fn latch_clock_data(&mut self, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        if let Some(clock) = self.clock.as_ref() {
            self.regs.rtc = clock.into();
        } else {
            return Err(Error::new_segfault(addr));
        }
        Ok(())
    }

    fn read_ram(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        if !self.regs.ram_enabled {
            return Err(Error::new_segfault(addr));
        }
        let address = addr.get_address();
        let ram_bank = self.regs.ram_bank;
        match ram_bank {
            0x0..=0x3 => Ok(self.ram_banks[ram_bank as usize][(address as usize) & 0x1FFF]),
            0x8 => Ok(self.regs.rtc.seconds),
            0x9 => Ok(self.regs.rtc.minutes),
            0xA => Ok(self.regs.rtc.hours),
            0xB => Ok(self.regs.rtc.lower_day_counter),
            0xC => Ok(self.regs.rtc.upper_day_counter),
            _ => return Err(Error::new_segfault(addr)),
        }
    }

    fn write_ram(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        if !self.regs.ram_enabled {
            return Err(Error::new_segfault(addr));
        }
        let address = addr.get_address();
        let ram_bank = self.regs.ram_bank;
        match ram_bank {
            0x0..=0x3 => self.ram_banks[ram_bank as usize][(address as usize) & 0x1FFF] = v,
            0x8 => self.regs.rtc.seconds = v,
            0x9 => self.regs.rtc.minutes = v,
            0xA => self.regs.rtc.hours = v,
            0xB => self.regs.rtc.lower_day_counter = v,
            0xC => self.regs.rtc.upper_day_counter = v,
            _ => return Err(Error::new_segfault(addr)),
        }
        Ok(())
    }
}

impl FileOperation<Area> for MBC3 {
    fn read(&self, addr: Box<dyn Address<Area>>) -> Result<u8, Error> {
        match addr.area_type() {
            Area::Rom => self.read_rom(addr),
            Area::Ram => self.read_ram(addr),
            _ => Err(Error::new_bus_error(addr)),
        }
    }
    fn write(&mut self, v: u8, addr: Box<dyn Address<Area>>) -> Result<(), Error> {
        match addr.area_type() {
            Area::Rom => self.write_rom(v, addr),
            Area::Ram => self.write_ram(v, addr),
            _ => Err(Error::new_bus_error(addr)),
        }
    }
}

#[derive(Default)]
struct MBC3Regs {
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
    rtc: RTCRegs,
    last_writed_byte: Option<u8>,
}

#[derive(Default)]
struct RTCRegs {
    seconds: u8,
    minutes: u8,
    hours: u8,
    lower_day_counter: u8,
    upper_day_counter: u8,
}

impl<T: ReadRtcRegisters> From<&T> for RTCRegs {
    fn from(clock: &T) -> Self {
        Self {
            seconds: clock.seconds(),
            minutes: clock.minutes(),
            hours: clock.hours(),
            lower_day_counter: clock.lower_days(),
            upper_day_counter: clock.control(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Mbc3Data {
    ram_banks: Vec<Vec<u8>>,
}

impl From<Vec<[u8; MBC3::RAM_BANK_SIZE]>> for Mbc3Data {
    fn from(banks: Vec<[u8; MBC3::RAM_BANK_SIZE]>) -> Self {
        Self {
            ram_banks: banks.iter().map(|bank| bank.to_vec()).collect(),
        }
    }
}

impl Controller for MBC3 {
    fn save<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data = Mbc3Data::from(self.ram_banks.clone());
        data.serialize(serializer)
    }

    fn load<'de, D>(&mut self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        use std::convert::TryFrom;

        let data = Mbc3Data::deserialize(deserializer)?;
        self.ram_banks = data
            .ram_banks
            .into_iter()
            .map(<[u8; MBC3::RAM_BANK_SIZE]>::try_from)
            .collect::<Result<Vec<[u8; MBC3::RAM_BANK_SIZE]>, Vec<u8>>>()
            .map_err(|faulty| {
                Error::invalid_length(
                    faulty.len(),
                    &format!("a ram bank size of size {}", MBC3::RAM_BANK_SIZE).as_str(),
                )
            })?;
        Ok(())
    }
}
