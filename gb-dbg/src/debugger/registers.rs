use egui::{Ui, Color32, Vec2};
use egui::Label;

struct Register(char, u8);

pub struct RegisterEditor {

    registers: Vec<Register>,
}

impl Default for RegisterEditor {
    /// Mock for debug purposes
    fn default() -> Self {
        Self {
            registers: vec![
                Register('A', 0x01),
                Register('B', 0x22),
                Register('C', 0x0F),
                Register('D', 0x0F),
                Register('F', 0x0F),
                Register('G', 0x0F),
                Register('H', 0x0F),
            ],
        }
    }
}

impl RegisterEditor {
    pub fn draw(&self, ui: &mut Ui) {
        ui.label(Label::new("Register Editors").text_color(Color32::WHITE));
        egui::CollapsingHeader::new("🛠 Options")
            .id_source(55)
            .default_open(false)
            .show(ui, |ui| ui.label("Hello"));
        ui.separator();
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(2.0, 2.0);
            self.draw_register_table("CPU", ui);
            self.draw_register_table("PPU", ui);
        });
    }

    fn draw_register_table(&self, name: &str, ui: &mut Ui) {
        let layout = egui::Layout::top_down(egui::Align::LEFT);
        ui.allocate_ui_with_layout(Vec2::new(150.0, 150.0), layout, |ui| {
            egui::trace!(ui, "VERT");
            ui.colored_label(Color32::WHITE, name);
            ui.horizontal(|ui| {
                ui.colored_label(Color32::WHITE, "Name");
                ui.add_space(8.0);
                ui.colored_label(Color32::WHITE, "Value");
            });
            egui::ScrollArea::from_max_height(100.0).id_source("ScrollArea_".to_owned() + name).show(ui, |ui| {
                egui::trace!(ui, "SCROLL");
                egui::Grid::new("Grid_".to_owned() + name).striped(true).spacing(Vec2::new(3.0, 3.0)).show(ui, |ui| {
                    for row in &self.registers {
                        ui.label(egui::Label::new(row.0));
                        ui.label(egui::Label::new(format!("0x{:02X}", row.1)));
                        ui.end_row();
                    }
                    ui.end_row();
                });
            });
        });
    }
}
