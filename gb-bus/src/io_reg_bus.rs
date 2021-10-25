use crate::{
    address::Address,
    io_reg_constant::{
        BG_OBJ_PALETTES_END, BG_OBJ_PALETTES_START, BOOT_ROM_START, COMMUNICATION_END,
        COMMUNICATION_START, CONTROLLER_START, DIV_TIMER_START, LCD_END, LCD_START, SOUND_END,
        SOUND_START, TIMER_CONTROL_START, TIMER_COUNTER_START, TIMER_MODULO_START, VRAM_BANK_START,
        VRAM_DMA_END, VRAM_DMA_START, WAVEFORM_RAM_END, WAVEFORM_RAM_START, WRAM_BANK_START,
    },
    Address as PseudoAddress, Area, Error, FileOperation, IORegArea,
};

pub struct IORegBus {
    pub controller: Box<dyn FileOperation<IORegArea>>,
    pub communication: Box<dyn FileOperation<IORegArea>>,
    pub div_timer: Box<dyn FileOperation<IORegArea>>,
    pub sound: Box<dyn FileOperation<IORegArea>>,
    pub waveform_ram: Box<dyn FileOperation<IORegArea>>,
    pub lcd: Box<dyn FileOperation<IORegArea>>,
    pub vram_bank: Box<dyn FileOperation<IORegArea>>,
    pub boot_rom: Box<dyn FileOperation<IORegArea>>,
    pub vram_dma: Box<dyn FileOperation<IORegArea>>,
    pub bg_obj_palettes: Box<dyn FileOperation<IORegArea>>,
    pub wram_bank: Box<dyn FileOperation<IORegArea>>,
}

impl FileOperation<Area> for IORegBus {
    fn read(&self, address: Box<dyn PseudoAddress<Area>>) -> Result<u8, Error> {
        let addr: u16 = address.into();
        match addr {
            CONTROLLER_START => self.controller.read(Box::new(Address::from_offset(
                IORegArea::Controller,
                addr,
                COMMUNICATION_START,
            ))),
            COMMUNICATION_START..=COMMUNICATION_END => self.communication.read(Box::new(
                Address::from_offset(IORegArea::Communication, addr, COMMUNICATION_START),
            )),
            DIV_TIMER_START => self.div_timer.read(Box::new(Address::from_offset(
                IORegArea::DivTimer,
                addr,
                DIV_TIMER_START,
            ))),
            TIMER_COUNTER_START => self
                .tima
                .read(Box::new(Address::byte_reg(IORegArea::TimerCounter, addr))),
            TIMER_MODULO_START => self
                .tma
                .read(Box::new(Address::byte_reg(IORegArea::TimerModulo, addr))),
            TIMER_CONTROL_START => self
                .tac
                .read(Box::new(Address::byte_reg(IORegArea::TimerControl, addr))),
            SOUND_START..=SOUND_END => self.sound.read(Box::new(Address::from_offset(
                IORegArea::Sound,
                addr,
                SOUND_START,
            ))),
            WAVEFORM_RAM_START..=WAVEFORM_RAM_END => self.waveform_ram.read(Box::new(
                Address::from_offset(IORegArea::WaveformRam, addr, WAVEFORM_RAM_START),
            )),
            LCD_START..=LCD_END => self.lcd.read(Box::new(Address::from_offset(
                IORegArea::Lcd,
                addr,
                LCD_START,
            ))),
            VRAM_BANK_START => self.vram_bank.read(Box::new(Address::from_offset(
                IORegArea::VRamBank,
                addr,
                VRAM_BANK_START,
            ))),
            BOOT_ROM_START => self.boot_rom.read(Box::new(Address::from_offset(
                IORegArea::BootRom,
                addr,
                BOOT_ROM_START,
            ))),
            VRAM_DMA_START..=VRAM_DMA_END => self.vram_dma.read(Box::new(Address::from_offset(
                IORegArea::VramDma,
                addr,
                VRAM_DMA_START,
            ))),
            BG_OBJ_PALETTES_START..=BG_OBJ_PALETTES_END => self.bg_obj_palettes.read(Box::new(
                Address::from_offset(IORegArea::BgObjPalettes, addr, BG_OBJ_PALETTES_START),
            )),
            WRAM_BANK_START => self.wram_bank.read(Box::new(Address::from_offset(
                IORegArea::WRamBank,
                addr,
                WRAM_BANK_START,
            ))),
            _ => Err(Error::BusError(addr)),
        }
    }

    fn write(&mut self, v: u8, address: Box<dyn PseudoAddress<Area>>) -> Result<(), Error> {
        let addr: u16 = address.into();
        match addr {
            CONTROLLER_START => self.controller.write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::Controller,
                    addr,
                    CONTROLLER_START,
                )),
            ),
            COMMUNICATION_START..=COMMUNICATION_END => self.communication.write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::Communication,
                    addr,
                    COMMUNICATION_START,
                )),
            ),
            DIV_TIMER_START => self
                .div_timer
                .write(v, Box::new(Address::byte_reg(IORegArea::DivTimer, addr))),
            TIMER_COUNTER_START => self.tima.write(
                v,
                Box::new(Address::byte_reg(IORegArea::TimerCounter, addr)),
            ),
            TIMER_MODULO_START => self
                .tma
                .write(v, Box::new(Address::byte_reg(IORegArea::TimerModulo, addr))),
            TIMER_CONTROL_START => self.tac.write(
                v,
                Box::new(Address::byte_reg(IORegArea::TimerControl, addr)),
            ),
            SOUND_START..=SOUND_END => self.sound.write(
                v,
                Box::new(Address::from_offset(IORegArea::Sound, addr, SOUND_START)),
            ),
            WAVEFORM_RAM_START..=WAVEFORM_RAM_END => self.waveform_ram.write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::WaveformRam,
                    addr,
                    WAVEFORM_RAM_START,
                )),
            ),
            LCD_START..=LCD_END => self.lcd.write(
                v,
                Box::new(Address::from_offset(IORegArea::Lcd, addr, LCD_START)),
            ),
            VRAM_BANK_START => self.vram_bank.write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::VRamBank,
                    addr,
                    VRAM_BANK_START,
                )),
            ),
            BOOT_ROM_START => self.boot_rom.write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::BootRom,
                    addr,
                    BOOT_ROM_START,
                )),
            ),
            VRAM_DMA_START..=VRAM_DMA_END => self.vram_dma.write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::VramDma,
                    addr,
                    VRAM_BANK_START,
                )),
            ),
            BG_OBJ_PALETTES_START..=BG_OBJ_PALETTES_END => self.bg_obj_palettes.write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::BgObjPalettes,
                    addr,
                    BG_OBJ_PALETTES_START,
                )),
            ),
            WRAM_BANK_START => self.wram_bank.write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::WRamBank,
                    addr,
                    WRAM_BANK_START,
                )),
            ),
            _ => Err(Error::BusError(addr)),
        }
    }
}
