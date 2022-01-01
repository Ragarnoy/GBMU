#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[repr(u16)]
pub enum IORegArea {
    /// Joypad
    Joy = 0xff00,
    /// Serial Byte
    SB = 0xff01,
    /// Serial Control
    SC = 0xff02,

    /// Clock divider
    Div = 0xff04,
    /// Timer Value
    Tima = 0xff05,
    /// Timer Reload
    Tma = 0xff06,
    /// Timer Control
    Tac = 0xff07,

    /// Interrupt flag
    IF = 0xff0f,

    /// Audio channel 1 sweep
    Nr10 = 0xff10,
    /// Audio channel 1 sound length/wave duty
    Nr11 = 0xff11,
    /// Audio channel 1 envelope
    Nr12 = 0xff12,
    /// Audio channel 1 frequency
    Nr13 = 0xff13,
    /// Audio channel 1 control
    Nr14 = 0xff14,

    /// Audio channel 2 sound length/wave duty
    Nr21 = 0xff16,
    /// Audio channel 2 envelope
    Nr22 = 0xff17,
    /// Audio channel 2 frequency
    Nr23 = 0xff18,
    /// Audio channel 2 control
    Nr24 = 0xff19,

    /// Audio channel 3 enable
    Nr30 = 0xff1a,
    /// Audio channel 3 sound length
    Nr31 = 0xff1b,
    /// Audio channel 3 volume
    Nr32 = 0xff1c,
    /// Audio channel 3 frequency
    Nr33 = 0xff1d,
    /// Audio channel 3 control
    Nr34 = 0xff1e,

    /// Audio channel 4 sound length
    Nr41 = 0xff20,
    /// Audio channel 4 volume
    Nr42 = 0xff21,
    /// Audio channel 4 frequency
    Nr43 = 0xff22,
    /// Audio channel 4 control
    Nr44 = 0xff23,

    /// Audio output mapping
    Nr50 = 0xff24,
    /// Audio channel mapping
    Nr51 = 0xff25,
    /// Audio channel control
    Nr52 = 0xff26,

    /// Wave pattern RAM byte 0
    WaveRam0 = 0xff30,
    /// Wave pattern RAM byte 1
    WaveRam1 = 0xff31,
    /// Wave pattern RAM byte 2
    WaveRam2 = 0xff32,
    /// Wave pattern RAM byte 3
    WaveRam3 = 0xff33,
    /// Wave pattern RAM byte 4
    WaveRam4 = 0xff34,
    /// Wave pattern RAM byte 5
    WaveRam5 = 0xff35,
    /// Wave pattern RAM byte 6
    WaveRam6 = 0xff36,
    /// Wave pattern RAM byte 7
    WaveRam7 = 0xff37,
    /// Wave pattern RAM byte 8
    WaveRam8 = 0xff38,
    /// Wave pattern RAM byte 9
    WaveRam9 = 0xff39,
    /// Wave pattern RAM byte A
    WaveRamA = 0xff3a,
    /// Wave pattern RAM byte B
    WaveRamB = 0xff3b,
    /// Wave pattern RAM byte C
    WaveRamC = 0xff3c,
    /// Wave pattern RAM byte D
    WaveRamD = 0xff3d,
    /// Wave pattern RAM byte E
    WaveRamE = 0xff3e,
    /// Wave pattern RAM byte F
    WaveRamF = 0xff3f,

    /// LCD control
    LcdControl = 0xff40,
    /// LCD status
    LcdStat = 0xff41,
    /// Background vertical scroll
    Scy = 0xff42,
    /// Background horizontal scrool
    Scx = 0xff43,
    /// LCD Y coordinate
    Ly = 0xff44,
    /// LCD Y compare
    Lyc = 0xff45,
    /// OAM DMA source address
    Dma = 0xff46,
    /// Background palette
    Bgp = 0xff47,
    /// OBJ palette 0
    Obp0 = 0xff48,
    /// OBJ palette 1
    Obp1 = 0xff49,
    Wy = 0xff4a,
    Wx = 0xff4b,
    #[cfg(feature = "cgb")]
    /// Prepare speed switch
    Key1 = 0xff4d,
    #[cfg(feature = "cgb")]
    /// Vram Bank
    Vbk = 0xff4f,

    BootRom = 0xff50,

    #[cfg(feature = "cgb")]
    /// New DMA source high
    Hdma1 = 0xff51,
    #[cfg(feature = "cgb")]
    /// New DMA source low
    Hdma2 = 0xff52,
    #[cfg(feature = "cgb")]
    /// New DMA destination high
    Hdma3 = 0xff53,
    #[cfg(feature = "cgb")]
    /// New DMA destination low
    Hdma4 = 0xff54,
    #[cfg(feature = "cgb")]
    /// New DMA Length/Mode/Start
    Hdma5 = 0xff55,
    #[cfg(feature = "cgb")]
    /// Infrared communication port
    RP = 0xff56,
    #[cfg(feature = "cgb")]
    /// Object priority mode
    Opri = 0xff6c,
    #[cfg(feature = "cgb")]
    /// Wram Bank
    Svbk = 0xff70,
}

impl From<IORegArea> for u16 {
    fn from(r: IORegArea) -> Self {
        r.into()
    }
}

impl TryFrom<u16> for IORegArea {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use crate::io_reg_constant::*;

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
