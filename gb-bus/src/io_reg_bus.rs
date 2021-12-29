#[cfg(feature = "cgb")]
use crate::io_reg_constant::{VRAM_BANK_START, VRAM_DMA_END, VRAM_DMA_START, WRAM_BANK_START};
use crate::{
    io_reg_constant::{
        BG_OBJ_PALETTES_END, BG_OBJ_PALETTES_START, BOOT_ROM_START, COMMUNICATION_END,
        COMMUNICATION_START, CONTROLLER_START, DIV_TIMER_START, INTERRUPT_FLAG, LCD_END, LCD_START,
        OAM_DMA_START, SOUND_END, SOUND_START, TIMER_CONTROL_START, TIMER_COUNTER_START,
        TIMER_MODULO_START, WAVEFORM_RAM_END, WAVEFORM_RAM_START,
    },
    Addr, Address, Area, Error, FileOperation, IORegArea,
};
use std::{cell::RefCell, rc::Rc};

macro_rules! match_area {
    ($sub_macro:ident, $self:expr, $addr:expr $(,$args:expr)*) => {
        match $addr {
            CONTROLLER_START => $sub_macro!(CONTROLLER_START, $self.controller, Controller, $addr $(,$args)*),
            COMMUNICATION_START..=COMMUNICATION_END => {
                $sub_macro!(COMMUNICATION_START, $self.communication, Communication, $addr $(,$args)*)
            }

            DIV_TIMER_START => $sub_macro!(DIV_TIMER_START, $self.div_timer, DivTimer, $addr $(,$args)*),
            TIMER_COUNTER_START => $sub_macro!(TIMER_COUNTER_START, $self.tima, TimerCounter, $addr $(,$args)*),
            TIMER_MODULO_START => $sub_macro!(TIMER_MODULO_START, $self.tma, TimerModulo, $addr $(,$args)*),
            TIMER_CONTROL_START => $sub_macro!(TIMER_CONTROL_START, $self.tac, TimerControl, $addr $(,$args)*),
            INTERRUPT_FLAG => $sub_macro!(INTERRUPT_FLAG, $self.interrupt_flag, InterruptFlag, $addr $(,$args)*),
            SOUND_START..=SOUND_END => $sub_macro!(SOUND_START, $self.sound, Sound, $addr $(,$args)*),
            WAVEFORM_RAM_START..=WAVEFORM_RAM_END => {
                $sub_macro!(WAVEFORM_RAM_START, $self.waveform_ram, WaveformRam, $addr $(,$args)*)
            }
            OAM_DMA_START => $sub_macro!(OAM_DMA_START, $self.oam_dma, OamDma, $addr $(,$args)*),
            LCD_START..=LCD_END => $sub_macro!(LCD_START, $self.lcd, Lcd, $addr $(,$args)*),
            #[cfg(feature = "cgb")]
            VRAM_BANK_START => $sub_macro!(VRAM_BANK_START, $self.vram_bank, VRamBank, $addr $(,$args)*),
            BOOT_ROM_START => $sub_macro!(BOOT_ROM_START, $self.boot_rom, BootRom, $addr $(,$args)*),
            #[cfg(feature = "cgb")]
            VRAM_DMA_START..=VRAM_DMA_END => {
                $sub_macro!(VRAM_DMA_START, $self.vram_dma, VramDma, $addr $(,$args)*)
            }
            BG_OBJ_PALETTES_START..=BG_OBJ_PALETTES_END => {
                $sub_macro!(
                    BG_OBJ_PALETTES_START,
                    $self.bg_obj_palettes,
                    BgObjPalettes,
                    $addr $(,$args)*
                )
            }
            #[cfg(feature = "cgb")]
            WRAM_BANK_START => $sub_macro!(WRAM_BANK_START, $self.wram_bank, WRamBank, $addr $(,$args)*),
            _ => Err(Error::BusError($addr)),
        }
    };
}

macro_rules! write_area {
    ($start:expr, $field:expr, $area_type:ident, $addr:expr, $value:expr) => {{
        #[cfg(feature = "trace_bus_write")]
        log::trace!(
            "writing at {:4x} the value {:2x} in area {:?}",
            $addr,
            $value,
            IORegArea::$area_type
        );
        $field.borrow_mut().write(
            $value,
            Addr::from_offset(IORegArea::$area_type, $addr, $start),
        )
    }};
}

macro_rules! read_area {
    ($start:expr, $field:expr, $area_type:ident, $addr: expr) => {{
        #[cfg(feature = "trace_bus_rea")]
        log::trace!(
            "reading at {:4x} in area {:?}",
            $addr,
            IORegArea::$area_type
        );
        $field
            .borrow()
            .read(Addr::from_offset(IORegArea::$area_type, $addr, $start))
    }};
}

pub struct IORegBus {
    pub controller: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub communication: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub div_timer: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub interrupt_flag: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub tima: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub tma: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub tac: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub sound: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub waveform_ram: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub lcd: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub oam_dma: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    #[cfg(feature = "cgb")]
    pub vram_bank: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub boot_rom: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    #[cfg(feature = "cgb")]
    pub vram_dma: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    pub bg_obj_palettes: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
    #[cfg(feature = "cgb")]
    pub wram_bank: Rc<RefCell<dyn FileOperation<Addr<IORegArea>, IORegArea>>>,
}

impl<A> FileOperation<A, Area> for IORegBus
where
    u16: From<A>,
    A: Address<Area>,
{
    fn read(&self, address: A) -> Result<u8, Error> {
        let addr: u16 = address.into();
        match_area!(read_area, self, addr)
    }

    fn write(&mut self, v: u8, address: A) -> Result<(), Error> {
        let addr: u16 = address.into();
        match_area!(write_area, self, addr, v)
    }
}
