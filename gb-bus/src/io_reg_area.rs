#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[repr(u16)]
pub enum IORegArea {
    /// Joypad
    JOY = 0xff00,
    /// Serial Byte
    SB = 0xff01,
    /// Serial Control
    SC = 0xff02,

    /// Clock divider
    Div = 0xff04,
    /// Timer Value
    TIMA = 0xff05,
    /// Timer Reload
    TMA = 0xff06,
    /// Timer Control
    TAC = 0xff07,

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
    SCY = 0xff42,
    /// Background horizontal scrool
    SCX = 0xff43,
    /// LCD Y coordinate
    LY = 0xff44,
    /// LCD Y compare
    LYC = 0xff45,
    /// OAM DMA source address
    DMA = 0xff46,
    /// Background palette
    BGP = 0xff47,
    /// OBJ palette 0
    OBP0 = 0xff48,
    /// OBJ palette 1
    OBP1 = 0xff49,
    WY = 0xff4a,
    WX = 0xff4b,
    #[cfg(feature = "cgb")]
    /// Prepare speed switch
    KEY1 = 0xff4d,
    #[cfg(feature = "cgb")]
    /// Vram Bank
    VBK = 0xff4f,

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
    OPRI = 0xff6c,
    #[cfg(feature = "cgb")]
    /// Wram Bank
    SVBK = 0xff70,
}
