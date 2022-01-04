/// Joypad
pub const JOY: u16 = 0xff00;
/// Serial Byte
pub const SB: u16 = 0xff01;
/// Serial Control
pub const SC: u16 = 0xff02;

/// Clock divider
pub const DIV: u16 = 0xff04;
/// Timer Value
pub const TIMA: u16 = 0xff05;
/// Timer Reload
pub const TMA: u16 = 0xff06;
/// Timer Control
pub const TAC: u16 = 0xff07;

/// Interrupt flag
pub const IF: u16 = 0xff0f;

/// Audio channel 1 sweep
pub const NR10: u16 = 0xff10;
/// Audio channel 1 sound length/wave duty
pub const NR11: u16 = 0xff11;
/// Audio channel 1 envelope
pub const NR12: u16 = 0xff12;
/// Audio channel 1 frequency
pub const NR13: u16 = 0xff13;
/// Audio channel 1 control
pub const NR14: u16 = 0xff14;

/// Audio channel 2 sound length/wave duty
pub const NR21: u16 = 0xff16;
/// Audio channel 2 envelope
pub const NR22: u16 = 0xff17;
/// Audio channel 2 frequency
pub const NR23: u16 = 0xff18;
/// Audio channel 2 control
pub const NR24: u16 = 0xff19;

/// Audio channel 3 enable
pub const NR30: u16 = 0xff1a;
/// Audio channel 3 sound length
pub const NR31: u16 = 0xff1b;
/// Audio channel 3 volume
pub const NR32: u16 = 0xff1c;
/// Audio channel 3 frequency
pub const NR33: u16 = 0xff1d;
/// Audio channel 3 control
pub const NR34: u16 = 0xff1e;

/// Audio channel 4 sound length
pub const NR41: u16 = 0xff20;
/// Audio channel 4 volume
pub const NR42: u16 = 0xff21;
/// Audio channel 4 frequency
pub const NR43: u16 = 0xff22;
/// Audio channel 4 control
pub const NR44: u16 = 0xff23;

/// Audio output mapping
pub const NR50: u16 = 0xff24;
/// Audio channel mapping
pub const NR51: u16 = 0xff25;
/// Audio channel control
pub const NR52: u16 = 0xff26;

/// Wave pattern RAM byte 0
pub const WAVE_RAM_0: u16 = 0xff30;
/// Wave pattern RAM byte 1
pub const WAVE_RAM_1: u16 = 0xff31;
/// Wave pattern RAM byte 2
pub const WAVE_RAM_2: u16 = 0xff32;
/// Wave pattern RAM byte 3
pub const WAVE_RAM_3: u16 = 0xff33;
/// Wave pattern RAM byte 4
pub const WAVE_RAM_4: u16 = 0xff34;
/// Wave pattern RAM byte 5
pub const WAVE_RAM_5: u16 = 0xff35;
/// Wave pattern RAM byte 6
pub const WAVE_RAM_6: u16 = 0xff36;
/// Wave pattern RAM byte 7
pub const WAVE_RAM_7: u16 = 0xff37;
/// Wave pattern RAM byte 8
pub const WAVE_RAM_8: u16 = 0xff38;
/// Wave pattern RAM byte 9
pub const WAVE_RAM_9: u16 = 0xff39;
/// Wave pattern RAM byte A
pub const WAVE_RAM_A: u16 = 0xff3a;
/// Wave pattern RAM byte B
pub const WAVE_RAM_B: u16 = 0xff3b;
/// Wave pattern RAM byte C
pub const WAVE_RAM_C: u16 = 0xff3c;
/// Wave pattern RAM byte D
pub const WAVE_RAM_D: u16 = 0xff3d;
/// Wave pattern RAM byte E
pub const WAVE_RAM_E: u16 = 0xff3e;
/// Wave pattern RAM byte F
pub const WAVE_RAM_F: u16 = 0xff3f;

/// LCD control
pub const LCD_CONTROL: u16 = 0xff40;
/// LCD status
pub const LCD_STAT: u16 = 0xff41;
/// Background vertical scroll
pub const SCY: u16 = 0xff42;
/// Background horizontal scrool
pub const SCX: u16 = 0xff43;
/// LCD Y coordinate
pub const LY: u16 = 0xff44;
/// LCD Y compare
pub const LYC: u16 = 0xff45;
/// OAM DMA source address
pub const DMA: u16 = 0xff46;
/// Background palette
pub const BGP: u16 = 0xff47;
/// OBJ palette 0
pub const OBP0: u16 = 0xff48;
/// OBJ palette 1
pub const OBP1: u16 = 0xff49;
pub const WY: u16 = 0xff4a;
pub const WX: u16 = 0xff4b;
#[cfg(feature = "cgb")]
/// Prepare speed switch
pub const KEY1: u16 = 0xff4d;
#[cfg(feature = "cgb")]
/// Vram Bank
pub const VBK: u16 = 0xff4f;

pub const BOOTROM: u16 = 0xff50;

#[cfg(feature = "cgb")]
/// New DMA source high
pub const HDMA1: u16 = 0xff51;
#[cfg(feature = "cgb")]
/// New DMA source low
pub const HDMA2: u16 = 0xff52;
#[cfg(feature = "cgb")]
/// New DMA destination high
pub const HDMA3: u16 = 0xff53;
#[cfg(feature = "cgb")]
/// New DMA destination low
pub const HDMA4: u16 = 0xff54;
#[cfg(feature = "cgb")]
/// New DMA Length/Mode/Start
pub const HDMA5: u16 = 0xff55;
#[cfg(feature = "cgb")]
/// Infrared communication port
pub const RP: u16 = 0xff56;
#[cfg(feature = "cgb")]
/// Background Color Palette Specification
pub const BCPS: u16 = 0xff68;
#[cfg(feature = "cgb")]
/// Background Color Palette Data
pub const BCPD: u16 = 0xff69;
#[cfg(feature = "cgb")]
/// Object Color Palette Specification
pub const OCPS: u16 = 0xff6a;
#[cfg(feature = "cgb")]
/// Object Color Palette Data
pub const OCPD: u16 = 0xff6b;
#[cfg(feature = "cgb")]
/// Object priority mode
pub const OPRI: u16 = 0xff6c;
#[cfg(feature = "cgb")]
/// Wram Bank
pub const SVBK: u16 = 0xff70;
