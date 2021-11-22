use crate::dbg_interfaces::{CpuRegs, DebugOperations};
use egui::{Color32, Ui};

const Z_FLAG: u16 = 0b1000_0000;
const N_FLAG: u16 = 0b0100_0000;
const H_FLAG: u16 = 0b0010_0000;
const C_FLAG: u16 = 0b0001_0000;

pub struct StatusBar;

impl StatusBar {
    pub fn draw<DBG: DebugOperations>(&self, ui: &mut Ui, regs: &DBG) {
        ui.vertical(|ui| {
            ui.colored_label(Color32::WHITE, "Status");
            ui.separator();
            ui.horizontal(|ui| {
                ui.colored_label(Color32::GOLD, "Flags: ");
                ui.colored_label(
                    Color32::WHITE,
                    format!("{}", display_flags(u16::from(regs.cpu_get(CpuRegs::AF)))),
                );
            });
        });
    }
}

fn display_flags(af_reg: u16) -> String {
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
    f_display.into_iter().collect()
}
