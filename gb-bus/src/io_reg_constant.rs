pub const CONTROLLER_START: u16 = 0xFF00;
pub const COMMUNICATION_START: u16 = 0xFF01;
pub const COMMUNICATION_END: u16 = 0xFF02;
pub const DIV_TIMER_START: u16 = 0xFF04;
pub const INTERRUPT_FLAG: u16 = 0xFF0F;
pub const TIMER_COUNTER_START: u16 = 0xFF05;
pub const TIMER_MODULO_START: u16 = 0xFF06;
pub const TIMER_CONTROL_START: u16 = 0xFF07;
pub const SOUND_START: u16 = 0xFF10;
pub const SOUND_END: u16 = 0xFF26;
pub const WAVEFORM_RAM_START: u16 = 0xFF30;
pub const WAVEFORM_RAM_END: u16 = 0xFF3F;
pub const OAM_DMA_START: u16 = 0xFF46;
pub const LCD_START: u16 = 0xFF40;
pub const LCD_END: u16 = 0xFF4B;
pub const VRAM_BANK_START: u16 = 0xFF4F;
pub const BOOT_ROM_START: u16 = 0xFF50;
pub const VRAM_DMA_START: u16 = 0xFF51;
pub const VRAM_DMA_END: u16 = 0xFF55;
pub const BG_OBJ_PALETTES_START: u16 = 0xFF68;
pub const BG_OBJ_PALETTES_END: u16 = 0xFF69;
pub const WRAM_BANK_START: u16 = 0xFF70;

pub const PPU_CONTROL: u16 = 0xFF40;
pub const PPU_STATUS: u16 = 0xFF41;
pub const PPU_SCY: u16 = 0xFF42;
pub const PPU_SCX: u16 = 0xFF43;
pub const PPU_LY: u16 = 0xFF44;
pub const PPU_LYC: u16 = 0xFF45;
pub const PPU_DMA: u16 = 0xFF46;
pub const PPU_BGP: u16 = 0xFF47;
pub const PPU_OBP0: u16 = 0xFF48;
pub const PPU_OBP1: u16 = 0xFF49;
pub const PPU_WY: u16 = 0xFF4A;
pub const PPU_WX: u16 = 0xFF4B;

// joypad regs
pub const IO_JOY: u16 = 0xFF00;
// serial regs
pub const IO_SERIALBYTE: u16 = 0xFF01;
pub const IO_SERIALCTL: u16 = 0xFF02;
// Timer regs
pub const IO_DIV: u16 = 0xFF04;
pub const IO_TIMA: u16 = 0xFF05;
pub const IO_TMA: u16 = 0xFF06;
pub const IO_TAC: u16 = 0xFF07;
// cpu int regs
pub const IO_IF: u16 = 0xFF0F;
pub const IO_IE: u16 = 0xFFFF;
// Boot ROM
pub const IO_BOOTROM: u16 = 0xFF50;

pub const AUD_FS1: u16 = 0xFF10;
pub const AUD_PWM1: u16 = 0xFF11;
pub const AUD_ENV1: u16 = 0xFF12;
pub const AUD_AF1: u16 = 0xFF13;
pub const AUD_CTL1: u16 = 0xFF14;
pub const AUD_PWM2: u16 = 0xFF16;
pub const AUD_ENV2: u16 = 0xFF17;
pub const AUD_AF2: u16 = 0xFF18;
pub const AUD_CTL2: u16 = 0xFF19;
pub const AUD_A3TOGGLE: u16 = 0xFF1A;
pub const AUD_PWM3: u16 = 0xFF1B;
pub const AUD_VOL3: u16 = 0xFF1C;
pub const AUD_AF3: u16 = 0xFF1D;
pub const AUD_CTL3: u16 = 0xFF1E;
pub const AUD_PWM4: u16 = 0xFF20;
pub const AUD_VOL4: u16 = 0xFF21;
pub const AUD_AF4: u16 = 0xFF22;
pub const AUD_CTL4: u16 = 0xFF23;
pub const AUD_OUTPUT_MAP: u16 = 0xFF24;
pub const AUD_MAP: u16 = 0xFF25;
pub const AUD_CHANNEL_CTL: u16 = 0xFF26;
pub const AUD_WAVE: u16 = 0xFF30;
