#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum IORegArea {
    Controller,
    Communication,
    DivTimer,
    InterruptFlag,
    TimerCounter,
    TimerModulo,
    TimerControl,
    Sound,
    WaveformRam,
    OamDma,
    Lcd,
    #[cfg(feature = "cgb")]
    VRamBank,
    BootRom,
    #[cfg(feature = "cgb")]
    VramDma,
    BgObjPalettes,
    #[cfg(feature = "cgb")]
    WRamBank,
    #[cfg(feature = "cgb")]
    DoubleSpeed,
    #[cfg(feature = "cgb")]
    ObjectPriorityMode,
}

impl std::convert::From<IORegArea> for u16 {
    fn from(area: IORegArea) -> Self {
        use crate::io_reg_constant::{
            BG_OBJ_PALETTES_START, BOOT_ROM_START, COMMUNICATION_START, CONTROLLER_START,
            DIV_TIMER_START, INTERRUPT_FLAG, LCD_START, OAM_DMA_START, SOUND_START,
            TIMER_CONTROL_START, TIMER_COUNTER_START, TIMER_MODULO_START, WAVEFORM_RAM_START,
        };
        #[cfg(feature = "cgb")]
        use crate::io_reg_constant::{KEY1, OPRI, VRAM_BANK_START, VRAM_DMA_START, WRAM_BANK};

        match area {
            IORegArea::Controller => CONTROLLER_START,
            IORegArea::Communication => COMMUNICATION_START,
            IORegArea::DivTimer => DIV_TIMER_START,
            IORegArea::InterruptFlag => INTERRUPT_FLAG,
            IORegArea::TimerCounter => TIMER_COUNTER_START,
            IORegArea::TimerModulo => TIMER_MODULO_START,
            IORegArea::TimerControl => TIMER_CONTROL_START,
            IORegArea::Sound => SOUND_START,
            IORegArea::WaveformRam => WAVEFORM_RAM_START,
            IORegArea::OamDma => OAM_DMA_START,
            IORegArea::Lcd => LCD_START,
            #[cfg(feature = "cgb")]
            IORegArea::VRamBank => VRAM_BANK_START,
            IORegArea::BootRom => BOOT_ROM_START,
            #[cfg(feature = "cgb")]
            IORegArea::VramDma => VRAM_DMA_START,
            IORegArea::BgObjPalettes => BG_OBJ_PALETTES_START,
            #[cfg(feature = "cgb")]
            IORegArea::WRamBank => WRAM_BANK,
            #[cfg(feature = "cgb")]
            IORegArea::DoubleSpeed => KEY1,
            #[cfg(feature = "cgb")]
            IORegArea::ObjectPriorityMode => OPRI,
        }
    }
}
