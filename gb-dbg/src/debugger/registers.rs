use crate::dbg_interfaces::{RegisterDebugOperations, RegisterMap};

use egui::Label;
use egui::{CollapsingHeader, Color32, Ui, Vec2};

pub struct RegisterEditor;

impl RegisterEditor {
    pub fn draw<REG: RegisterDebugOperations>(&mut self, ui: &mut Ui, register: &REG) {
        ui.label(Label::new("Register Editors").text_color(Color32::WHITE));
        CollapsingHeader::new("🛠 Options")
            .id_source("Register_Options")
            .default_open(false)
            .show(ui, |ui| {
                ui.label("Hello");
            });
        ui.separator();
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(2.0, 2.0);
            self.draw_register_table(register.cpu_registers(), "CPU", ui);
            self.draw_register_table(register.ppu_registers(), "PPU", ui);
            self.draw_register_table(register.io_registers(), "IO", ui);
            self.draw_register_table(register.audio_registers(), "AUDIO", ui);
        });
        ui.add_space(58.0);
        ui.separator();
    }

    fn draw_register_table<T: std::fmt::Display + std::fmt::Debug>(
        &self,
        registers: Vec<RegisterMap<T>>,
        name: &str,
        ui: &mut Ui,
    ) {
        let layout = egui::Layout::top_down(egui::Align::LEFT);
        ui.allocate_ui_with_layout(Vec2::new(125.0, 150.0), layout, |ui| {
            ui.colored_label(Color32::WHITE, name);
            ui.horizontal(|ui| {
                ui.colored_label(Color32::WHITE, "Name");
                ui.add_space(8.0);
                ui.colored_label(Color32::WHITE, "Value");
            });
            egui::ScrollArea::from_max_height(400.0)
                .id_source("ScrollArea_".to_owned() + name)
                .show(ui, |ui| {
                    egui::Grid::new("Grid_".to_owned() + name)
                        .striped(true)
                        .spacing(Vec2::new(3.5, 2.5))
                        .show(ui, |ui| {
                            for row in registers.iter() {
                                let value: u16 = row.1.into();
                                let format = if value > u8::MAX as u16 {
                                    format!("0x{:04X}", value)
                                } else {
                                    format!("0x{:02X}", value)
                                };

                                ui.label(egui::Label::new(format!("{:?}", &row.0)))
                                    .on_hover_text(&row.0);
                                ui.label(egui::Label::new(format));
                                ui.end_row();
                            }
                            ui.end_row();
                        });
                });
        });
    }
}
