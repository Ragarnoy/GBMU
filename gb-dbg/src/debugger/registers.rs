use crate::dbg_interfaces::{RegisterDebugOperations, RegisterMap, RegisterValue};

use egui::Label;
use egui::{Color32, Ui, Vec2};

pub struct RegisterEditor;

impl RegisterEditor {
    pub fn draw<REG: RegisterDebugOperations>(&mut self, ui: &mut Ui, register: &REG) {
        ui.vertical(|ui| {
            ui.label(Label::new("Registers").text_color(Color32::LIGHT_BLUE));
            ui.separator();
            ui.add_space(8.0);
            ui.horizontal_top(|ui| {
                ui.spacing_mut().window_padding = Vec2::new(16.0, 16.0);

                ui.spacing_mut().item_spacing = Vec2::new(16.0, 2.0);
                self.draw_register_table(register.cpu_registers(), "CPU", ui);

                ui.separator();
                self.draw_register_table(register.io_registers(), "IO", ui);

                ui.separator();

                self.draw_register_table(register.ppu_registers(), "PPU", ui);

                ui.separator();
                self.draw_register_table(register.audio_registers(), "AUDIO", ui);
            });
        });
    }

    fn draw_register_table<T: std::fmt::Display + std::fmt::Debug>(
        &self,
        registers: Vec<RegisterMap<T>>,
        name: &str,
        ui: &mut Ui,
    ) {
        let layout = egui::Layout::top_down(egui::Align::LEFT);
        ui.allocate_ui_with_layout(Vec2::new(125.0, 300.0), layout, |ui| {
            ui.colored_label(Color32::GOLD, name);
            ui.separator();
            egui::ScrollArea::from_max_height(240.0)
                .id_source("ScrollArea_".to_owned() + name)
                .show(ui, |ui| {
                    egui::Grid::new("Grid_".to_owned() + name)
                        .striped(true)
                        .show(ui, |ui| {
                            for row in registers.iter() {
                                let format = match row.1 {
                                    RegisterValue::U8(v) => format!("0x{:02X}", v),
                                    RegisterValue::U16(v) => format!("0x{:04X}", v),
                                };

                                // Only display tooltip if tooltip string is different from register name
                                if format!("{}", &row.0).to_lowercase()
                                    != format!("{:?}", &row.0).to_lowercase()
                                {
                                    ui.colored_label(
                                        Color32::WHITE,
                                        egui::Label::new(format!("{:?}", &row.0)),
                                    )
                                    .on_hover_text(&row.0);
                                } else {
                                    ui.colored_label(
                                        Color32::WHITE,
                                        egui::Label::new(format!("{}", &row.0)),
                                    );
                                }
                                ui.colored_label(Color32::WHITE, egui::Label::new(format));
                                ui.end_row();
                            }
                        });
                });
        });
    }
}
