use crate::io_reg_constant::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[repr(u16)]
pub enum IORegArea {
    /// Joypad
    Joy = JOY,
    /// Serial Byte
    SB = SB,
    /// Serial Control
    SC = SC,

    /// Clock divider
    Div = DIV,
    /// Timer Value
    Tima = TIMA,
    /// Timer Reload
    Tma = TMA,
    /// Timer Control
    Tac = TAC,

    /// Interrupt flag
    IF = IF,

    /// Audio channel 1 sweep
    Nr10 = NR10,
    /// Audio channel 1 sound length/wave duty
    Nr11 = NR11,
    /// Audio channel 1 envelope
    Nr12 = NR12,
    /// Audio channel 1 frequency
    Nr13 = NR13,
    /// Audio channel 1 control
    Nr14 = NR14,

    /// Audio channel 2 sound length/wave duty
    Nr21 = NR21,
    /// Audio channel 2 envelope
    Nr22 = NR22,
    /// Audio channel 2 frequency
    Nr23 = NR23,
    /// Audio channel 2 control
    Nr24 = NR24,

    /// Audio channel 3 enable
    Nr30 = NR30,
    /// Audio channel 3 sound length
    Nr31 = NR31,
    /// Audio channel 3 volume
    Nr32 = NR32,
    /// Audio channel 3 frequency
    Nr33 = NR33,
    /// Audio channel 3 control
    Nr34 = NR34,

    /// Audio channel 4 sound length
    Nr41 = NR41,
    /// Audio channel 4 volume
    Nr42 = NR42,
    /// Audio channel 4 frequency
    Nr43 = NR43,
    /// Audio channel 4 control
    Nr44 = NR44,

    /// Audio output mapping
    Nr50 = NR50,
    /// Audio channel mapping
    Nr51 = NR51,
    /// Audio channel control
    Nr52 = NR52,

    /// Wave pattern RAM byte 0
    WaveRam0 = WAVE_RAM_0,
    /// Wave pattern RAM byte 1
    WaveRam1 = WAVE_RAM_1,
    /// Wave pattern RAM byte 2
    WaveRam2 = WAVE_RAM_2,
    /// Wave pattern RAM byte 3
    WaveRam3 = WAVE_RAM_3,
    /// Wave pattern RAM byte 4
    WaveRam4 = WAVE_RAM_4,
    /// Wave pattern RAM byte 5
    WaveRam5 = WAVE_RAM_5,
    /// Wave pattern RAM byte 6
    WaveRam6 = WAVE_RAM_6,
    /// Wave pattern RAM byte 7
    WaveRam7 = WAVE_RAM_7,
    /// Wave pattern RAM byte 8
    WaveRam8 = WAVE_RAM_8,
    /// Wave pattern RAM byte 9
    WaveRam9 = WAVE_RAM_9,
    /// Wave pattern RAM byte A
    WaveRamA = WAVE_RAM_A,
    /// Wave pattern RAM byte B
    WaveRamB = WAVE_RAM_B,
    /// Wave pattern RAM byte C
    WaveRamC = WAVE_RAM_C,
    /// Wave pattern RAM byte D
    WaveRamD = WAVE_RAM_D,
    /// Wave pattern RAM byte E
    WaveRamE = WAVE_RAM_E,
    /// Wave pattern RAM byte F
    WaveRamF = WAVE_RAM_F,

    /// LCD control
    LcdControl = LCD_CONTROL,
    /// LCD status
    LcdStat = LCD_STAT,
    /// Background vertical scroll
    Scy = SCY,
    /// Background horizontal scrool
    Scx = SCX,
    /// LCD Y coordinate
    Ly = LY,
    /// LCD Y compare
    Lyc = LYC,
    /// OAM DMA source address
    Dma = DMA,
    /// Background palette
    Bgp = BGP,
    /// OBJ palette 0
    Obp0 = OBP0,
    /// OBJ palette 1
    Obp1 = OBP1,
    Wy = WY,
    Wx = WX,
    #[cfg(feature = "cgb")]
    /// Prepare speed switch
    Key1 = KEY1,
    #[cfg(feature = "cgb")]
    /// Vram Bank
    Vbk = VBK,

    BootRom = BOOTROM,

    #[cfg(feature = "cgb")]
    /// New DMA source high
    Hdma1 = HDMA1,
    #[cfg(feature = "cgb")]
    /// New DMA source low
    Hdma2 = HDMA2,
    #[cfg(feature = "cgb")]
    /// New DMA destination high
    Hdma3 = HDMA3,
    #[cfg(feature = "cgb")]
    /// New DMA destination low
    Hdma4 = HDMA4,
    #[cfg(feature = "cgb")]
    /// New DMA Length/Mode/Start
    Hdma5 = HDMA5,
    #[cfg(feature = "cgb")]
    /// Infrared communication port
    RP = RP,
    #[cfg(feature = "cgb")]
    /// Object priority mode
    Opri = OPRI,
    #[cfg(feature = "cgb")]
    /// Wram Bank
    Svbk = SVBK,
}

impl From<IORegArea> for u16 {
    fn from(r: IORegArea) -> Self {
        r.into()
    }
}

impl TryFrom<u16> for IORegArea {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let result = match value {
            JOY => Self::Joy,
            SB => Self::SB,
            SC => Self::SC,
            DIV => Self::Div,
            TIMA => Self::Tima,
            TMA => Self::Tma,
            TAC => Self::Tac,
            IF => Self::IF,
            NR11 => Self::Nr11,
            NR12 => Self::Nr12,
            NR13 => Self::Nr13,
            NR14 => Self::Nr14,
            NR22 => Self::Nr22,
            NR23 => Self::Nr23,
            NR24 => Self::Nr24,
            NR31 => Self::Nr31,
            NR32 => Self::Nr32,
            NR33 => Self::Nr33,
            NR34 => Self::Nr34,
            NR42 => Self::Nr42,
            NR43 => Self::Nr43,
            NR44 => Self::Nr44,
            NR51 => Self::Nr51,
            NR52 => Self::Nr52,
            WAVE_RAM_1 => Self::WaveRam1,
            WAVE_RAM_2 => Self::WaveRam2,
            WAVE_RAM_3 => Self::WaveRam3,
            WAVE_RAM_4 => Self::WaveRam4,
            WAVE_RAM_5 => Self::WaveRam5,
            WAVE_RAM_6 => Self::WaveRam6,
            WAVE_RAM_7 => Self::WaveRam7,
            WAVE_RAM_8 => Self::WaveRam8,
            WAVE_RAM_9 => Self::WaveRam9,
            WAVE_RAM_A => Self::WaveRamA,
            WAVE_RAM_B => Self::WaveRamB,
            WAVE_RAM_C => Self::WaveRamC,
            WAVE_RAM_D => Self::WaveRamD,
            WAVE_RAM_E => Self::WaveRamE,
            WAVE_RAM_F => Self::WaveRamF,
            LCD_CONTROL => Self::LcdControl,
            LCD_STAT => Self::LcdStat,
            SCY => Self::Scy,
            SCX => Self::Scx,
            LY => Self::Ly,
            LYC => Self::Lyc,
            DMA => Self::Dma,
            BGP => Self::Bgp,
            OBP0 => Self::Obp0,
            OBP1 => Self::Obp1,
            WY => Self::Wy,
            WX => Self::Wx,
            #[cfg(feature = "cgb")]
            KEY1 => Self::Key1,
            #[cfg(feature = "cgb")]
            VBK => Self::Vbk,
            BOOTROM => Self::BootRom,
            #[cfg(feature = "cgb")]
            HDMA1 => Self::Hdma1,
            #[cfg(feature = "cgb")]
            HDMA2 => Self::Hdma2,
            #[cfg(feature = "cgb")]
            HDMA3 => Self::Hdma3,
            #[cfg(feature = "cgb")]
            HDMA4 => Self::Hdma4,
            #[cfg(feature = "cgb")]
            HDMA5 => Self::Hdma5,
            #[cfg(feature = "cgb")]
            RP => Self::RP,
            #[cfg(feature = "cgb")]
            OPRI => Self::Opri,
            #[cfg(feature = "cgb")]
            SVBK => Self::Svbk,
            _ => {
                return Err(format!("no register for {:x}", value));
            }
        };
        Ok(result)
    }
}
