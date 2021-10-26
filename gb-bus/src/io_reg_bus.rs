use crate::{
    address::Address,
    io_reg_constant::{
        BG_OBJ_PALETTES_END, BG_OBJ_PALETTES_START, BOOT_ROM_START, COMMUNICATION_END,
        COMMUNICATION_START, CONTROLLER_START, DIV_TIMER_START, INTERRUPT_FLAG, LCD_END, LCD_START,
        SOUND_END, SOUND_START, TIMER_CONTROL_START, TIMER_COUNTER_START, TIMER_MODULO_START,
        VRAM_BANK_START, VRAM_DMA_END, VRAM_DMA_START, WAVEFORM_RAM_END, WAVEFORM_RAM_START,
        WRAM_BANK_START,
    },
    Address as PseudoAddress, Area, Error, FileOperation, IORegArea,
};
use std::{cell::RefCell, rc::Rc};

macro_rules! write_area {
    ($start:expr, $field:expr, $area_type:ident, $value:expr, $addr:expr) => {{
        log::trace!(
            "writing at {:4x} the value {:2x} in area {:?}",
            $addr,
            $value,
            IORegArea::$area_type
        );
        $field.borrow_mut().write(
            $value,
            Box::new(Address::from_offset(IORegArea::$area_type, $addr, $start)),
        )
    }};
}

macro_rules! read_area {
    ($start:expr, $field:expr, $area_type:ident, $addr: expr) => {{
        log::trace!(
            "reading at {:4x} in area {:?}",
            $addr,
            IORegArea::$area_type
        );
        $field.borrow().read(Box::new(Address::from_offset(
            IORegArea::$area_type,
            $addr,
            $start,
        )))
    }};
}

pub struct IORegBus {
    pub controller: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub communication: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub div_timer: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub interrupt_flag: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub tima: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub tma: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub tac: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub sound: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub waveform_ram: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub lcd: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub vram_bank: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub boot_rom: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub vram_dma: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub bg_obj_palettes: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub wram_bank: Rc<RefCell<dyn FileOperation<IORegArea>>>,
}

impl FileOperation<Area> for IORegBus {
    fn read(&self, address: Box<dyn PseudoAddress<Area>>) -> Result<u8, Error> {
        let addr: u16 = address.into();
        match addr {
            CONTROLLER_START => read_area!(CONTROLLER_START, self.controller, Controller, addr),
            COMMUNICATION_START..=COMMUNICATION_END => {
                read_area!(COMMUNICATION_START, self.communication, Communication, addr)
            }

            DIV_TIMER_START => read_area!(DIV_TIMER_START, self.div_timer, DivTimer, addr),
            TIMER_COUNTER_START => read_area!(TIMER_COUNTER_START, self.tima, TimerCounter, addr),
            TIMER_MODULO_START => read_area!(TIMER_MODULO_START, self.tma, TimerModulo, addr),
            TIMER_CONTROL_START => read_area!(TIMER_CONTROL_START, self.tac, TimerControl, addr),
            INTERRUPT_FLAG => read_area!(INTERRUPT_FLAG, self.interrup_flag, InterruptFlag, addr),
            SOUND_START..=SOUND_END => read_area!(SOUND_START, self.sound, Sound, addr),
            WAVEFORM_RAM_START..=WAVEFORM_RAM_END => {
                read_area!(WAVEFORM_RAM_START, self.waveform_ram, WaveformRam, addr)
            }
            LCD_START..=LCD_END => read_area!(LCD_START, self.lcd, Lcd, addr),
            VRAM_BANK_START => read_area!(VRAM_BANK_START, self.vram_bank, VRamBank, addr),
            BOOT_ROM_START => read_area!(BOOT_ROM_START, self.boot_rom, BootRom, addr),
            VRAM_DMA_START..=VRAM_DMA_END => {
                read_area!(VRAM_DMA_START, self.vram_dma, VramDma, addr)
            }
            BG_OBJ_PALETTES_START..=BG_OBJ_PALETTES_END => {
                read_area!(
                    BG_OBJ_PALETTES_START,
                    self.bg_obj_palettes,
                    BgObjPalettes,
                    addr
                )
            }
            WRAM_BANK_START => read_area!(WRAM_BANK_START, self.wram_bank, WRamBank, addr),
            _ => Err(Error::BusError(addr)),
        }
    }

    fn write(&mut self, v: u8, address: Box<dyn PseudoAddress<Area>>) -> Result<(), Error> {
        let addr: u16 = address.into();
        match addr {
            CONTROLLER_START => write_area!(CONTROLLER_START, self.controller, Controller, v, addr),
            COMMUNICATION_START..=COMMUNICATION_END => write_area!(
                COMMUNICATION_START,
                self.communication,
                Communication,
                v,
                addr
            ),
            DIV_TIMER_START => write_area!(DIV_TIMER_START, self.div_timer, DivTimer, v, addr),
            TIMER_COUNTER_START => {
                write_area!(TIMER_COUNTER_START, self.tima, TimerCounter, v, addr)
            }
            TIMER_MODULO_START => write_area!(TIMER_MODULO_START, self.tma, TimerModulo, v, addr),
            TIMER_CONTROL_START => {
                write_area!(TIMER_CONTROL_START, self.tac, TimerControl, v, addr)
            }
            INTERRUPT_FLAG => {
                write_area!(INTERRUPT_FLAG, self.interrupt_flag, InterruptFlag, v, addr)
            }
            SOUND_START..=SOUND_END => write_area!(SOUND_START, self.sound, Sound, v, addr),
            WAVEFORM_RAM_START..=WAVEFORM_RAM_END => {
                write_area!(WAVEFORM_RAM_START, self.waveform_ram, WaveformRam, v, addr)
            }
            LCD_START..=LCD_END => write_area!(LCD_START, self.lcd, Lcd, v, addr),
            VRAM_BANK_START => write_area!(VRAM_BANK_START, self.vram_bank, VRamBank, v, addr),
            BOOT_ROM_START => write_area!(BOOT_ROM_START, self.boot_rom, BootRom, v, addr),
            VRAM_DMA_START..=VRAM_DMA_END => {
                write_area!(VRAM_BANK_START, self.vram_dma, VramDma, v, addr)
            }
            BG_OBJ_PALETTES_START..=BG_OBJ_PALETTES_END => write_area!(
                BG_OBJ_PALETTES_START,
                self.bg_obj_palettes,
                BgObjPalettes,
                v,
                addr
            ),
            WRAM_BANK_START => write_area!(WRAM_BANK_START, self.wram_bank, WRamBank, v, addr),
            _ => Err(Error::BusError(addr)),
        }
    }
}
