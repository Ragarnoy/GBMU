#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IORegArea {
    Controller,
    Communication,
    DivTimer,
    TimerCounter,
    TimerModulo,
    TimerControl,
    Sound,
    WaveformRam,
    Lcd,
    VRamBank,
    BootRom,
    VramDma,
    BgObjPalettes,
    WRamBank,
}

impl std::convert::From<IORegArea> for u16 {
    fn from(area: IORegArea) -> Self {
        use crate::io_reg_constant::{
            BG_OBJ_PALETTES_START, BOOT_ROM_START, COMMUNICATION_START, CONTROLLER_START,
            DIV_TIMER_START, LCD_START, SOUND_START, TIMER_CONTROL_START, TIMER_COUNTER_START,
            TIMER_MODULO_START, VRAM_BANK_START, VRAM_DMA_START, WAVEFORM_RAM_START,
            WRAM_BANK_START,
        };

        match area {
            IORegArea::Controller => CONTROLLER_START,
            IORegArea::Communication => COMMUNICATION_START,
            IORegArea::DivTimer => DIV_TIMER_START,
            IORegArea::TimerCounter => TIMER_COUNTER_START,
            IORegArea::TimerModulo => TIMER_MODULO_START,
            IORegArea::TimerControl => TIMER_CONTROL_START,
            IORegArea::Sound => SOUND_START,
            IORegArea::WaveformRam => WAVEFORM_RAM_START,
            IORegArea::Lcd => LCD_START,
            IORegArea::VRamBank => VRAM_BANK_START,
            IORegArea::BootRom => BOOT_ROM_START,
            IORegArea::VramDma => VRAM_DMA_START,
            IORegArea::BgObjPalettes => BG_OBJ_PALETTES_START,
            IORegArea::WRamBank => WRAM_BANK_START,
        }
    }
}
