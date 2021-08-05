pub const MBC1_RAM_BANK_MAX_SIZE: usize = 0x4000;

pub struct MBC1 {
    ram_enabled: bool,
    rom_bank_number: u8,
    /// can also be used for higher rom bank number
    /// depending on baking_mode
    ram_bank_number: u8,
    banking_mode: BankingMode,
}

enum BankingMode {
    Simple,
    Advanced,
}
