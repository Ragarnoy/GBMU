use crate::dbg_interfaces::{CpuRegs, DebugOperations};
use egui::{Color32, Ui};

const Z_FLAG: u16 = 0b1000_0000;
const N_FLAG: u16 = 0b0100_0000;
const H_FLAG: u16 = 0b0010_0000;
const C_FLAG: u16 = 0b0001_0000;

const RIGHT_OR_A: u16 = 0b0000_0001;
const LEFT_OR_B: u16 = 0b0000_0010;
const UP_OR_SELECT: u16 = 0b0000_0100;
const DOWN_OR_START: u16 = 0b0000_1000;
const SELECT_DIRECTION: u16 = 0b0001_0000;
const SELECT_ACTION: u16 = 0b0010_0000;

pub struct StatusBar;

impl StatusBar {
    pub fn draw<DBG: DebugOperations>(&self, ui: &mut Ui, regs: &DBG) {
        ui.vertical(|ui| {
            ui.colored_label(Color32::LIGHT_BLUE, "Status");
            ui.separator();
            ui.horizontal(|ui| {
                display_flags(ui, u16::from(regs.cpu_get(CpuRegs::AF)));
            });
        });
    }
}

fn display_key(ui: &mut Ui, key: u16) {
    let mut key_display: Vec<char> = vec!['_'; 6];
    if key & RIGHT_OR_A != 0 {
        key_display[0] = 'R';
    }
    if key & LEFT_OR_B != 0 {
        key_display[1] = 'L';
    }
    if key & UP_OR_SELECT != 0 {
        key_display[2] = 'U';
    }

}

fn display_flags(ui: &mut Ui, af_reg: u16) {
    let mut f_display: Vec<char> = vec!['_'; 4];
    if af_reg & Z_FLAG != 0 {
        f_display[0] = 'Z';
    }
    if af_reg & N_FLAG != 0 {
        f_display[1] = 'N';
    }
    if af_reg & H_FLAG != 0 {
        f_display[2] = 'H';
    }
    if af_reg & C_FLAG != 0 {
        f_display[3] = 'C';
    }
    ui.colored_label(Color32::GOLD, "Flags: ");
    ui.colored_label(Color32::WHITE, f_display.into_iter().collect::<String>());
}
