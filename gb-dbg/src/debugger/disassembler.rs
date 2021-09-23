use egui::{Color32, Label, Ui, Vec2};

pub struct DisassemblyViewer;

impl DisassemblyViewer {
    pub fn draw(&self, ui: &mut Ui, pc: u16) {
        ui.label(Label::new("Disassembler").text_color(Color32::WHITE));
        egui::CollapsingHeader::new("🛠 Options")
            .id_source(55)
            .default_open(false)
            .show(ui, |ui| ui.label("Hello"));
        ui.separator();
        ui.vertical(|ui| {
            egui::Grid::new("dissas_".to_owned())
                .striped(true)
                .spacing(Vec2::new(100.0, 2.5))
                .show(ui, |ui| {
                    ui.label(egui::Label::new("Address").text_color(Color32::WHITE));
                    ui.label(egui::Label::new("Instruction").text_color(Color32::WHITE));
                    ui.label(egui::Label::new("Data").text_color(Color32::WHITE));
                    ui.end_row();
                    for row in 0..8 {
                        ui.label(egui::Label::new(format!("0x{:04X}", pc + row)));
                        ui.label(egui::Label::new("add x and y"));
                        ui.label(egui::Label::new("0x00000000"));
                        ui.end_row();
                    }
                    ui.end_row();
                });
        });
    }
}
