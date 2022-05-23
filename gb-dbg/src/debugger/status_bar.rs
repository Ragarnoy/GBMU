use crate::dbg_interfaces::{CpuRegs, DebugOperations, IORegs};
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
    pub fn draw<DBG: DebugOperations>(
        &self,
        ui: &mut Ui,
        regs: &DBG,
        info: Option<(&dyn ToString, &dyn ToString)>,
    ) {
        ui.vertical(|ui| {
            ui.colored_label(Color32::LIGHT_BLUE, "Status");
            ui.separator();
            ui.horizontal(|ui| {
                display_flags(ui, u16::from(regs.cpu_get(CpuRegs::AF)));
                display_key(ui, u16::from(regs.io_get(IORegs::Joy)));
                if let Some(info_tuple) = info {
                    display_info(ui, info_tuple);
                }
            });
        });
    }
}

fn display_key(ui: &mut Ui, key: u16) {
    let mut key_display: Vec<&str> = vec!["___ "; 6];
    if key & RIGHT_OR_A == 0 {
        key_display[0] = "➡/A ";
    }
    if key & LEFT_OR_B == 0 {
        key_display[1] = "⬅/B ";
    }
    if key & UP_OR_SELECT == 0 {
        key_display[2] = "⬆/SL ";
    }
    if key & DOWN_OR_START == 0 {
        key_display[3] = "⬇/ST ";
    }
    if key & SELECT_DIRECTION == 0 {
        key_display[4] = "SL_D ";
    }
    if key & SELECT_ACTION == 0 {
        key_display[5] = "SL_A ";
    }
    ui.colored_label(Color32::GOLD, "Keys: ");
    ui.colored_label(Color32::WHITE, key_display.into_iter().collect::<String>());
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

fn display_info(ui: &mut Ui, info: (&dyn ToString, &dyn ToString)) {
    ui.colored_label(Color32::GOLD, format!("{}: ", info.0.to_string()));
    ui.colored_label(Color32::WHITE, info.1.to_string());
}
