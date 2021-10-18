use crate::{
    address::Address,
    io_reg_constant::{
        BG_OBJ_PALETTES_END, BG_OBJ_PALETTES_START, BOOT_ROM_START, COMMUNICATION_END,
        COMMUNICATION_START, CONTROLLER_START, DIV_TIMER_END, DIV_TIMER_START, INTERRUPT_FLAG,
        LCD_END, LCD_START, SOUND_END, SOUND_START, VRAM_BANK_START, VRAM_DMA_END, VRAM_DMA_START,
        WAVEFORM_RAM_END, WAVEFORM_RAM_START, WRAM_BANK_START,
    },
    Address as PseudoAddress, Area, Error, FileOperation, IORegArea,
};

struct IORegBus {
    controller: Box<dyn FileOperation<IORegArea>>,
    communication: Box<dyn FileOperation<IORegArea>>,
    div_timer: Box<dyn FileOperation<IORegArea>>,
    interrupt_flag: Box<dyn FileOperation<IORegArea>>,
    sound: Box<dyn FileOperation<IORegArea>>,
    waveform_ram: Box<dyn FileOperation<IORegArea>>,
    lcd: Box<dyn FileOperation<IORegArea>>,
    vram_bank: Box<dyn FileOperation<IORegArea>>,
    boot_rom: Box<dyn FileOperation<IORegArea>>,
    vram_dma: Box<dyn FileOperation<IORegArea>>,
    bg_obj_palettes: Box<dyn FileOperation<IORegArea>>,
    wram_bank: Box<dyn FileOperation<IORegArea>>,
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
            DIV_TIMER_START..=DIV_TIMER_END => self.div_timer.read(Box::new(Address::from_offset(
                IORegArea::DivTimer,
                addr,
                DIV_TIMER_START,
            ))),
            INTERRUPT_FLAG => self.interrupt_flag.read(Box::new(Address::from_offset(
                IORegArea::InterruptFlag,
                addr,
                INTERRUPT_FLAG,
            ))),
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
            DIV_TIMER_START..=DIV_TIMER_END => self.div_timer.write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::DivTimer,
                    addr,
                    DIV_TIMER_START,
                )),
            ),
            INTERRUPT_FLAG => self.interrupt_flag.write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::InterruptFlag,
                    addr,
                    INTERRUPT_FLAG,
                )),
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
