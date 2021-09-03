mod table;

use egui::Label;
use egui::{Color32, Ui, Vec2, CollapsingHeader};
use crate::debugger::registers::table::RegisterTable;
use crate::dbg_interfaces::DebugRegister;

pub struct RegisterEditor<T> {
    cpu: RegisterTable<T>,
    ppu: RegisterTable<T>,
    io: RegisterTable<T>,
}

impl<T> RegisterEditor<T> {
    pub fn draw(&self, ui: &mut Ui) {
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
            self.draw_register_table(&self.cpu, "CPU", ui);
            self.draw_register_table(&self.ppu, "PPU", ui);
            self.draw_register_table(&self.io, "IO", ui);
        });
        ui.add_space(58.0);
        ui.separator();
    }

    fn draw_register_table(&self, registers: &RegisterTable<T>, name: &str, ui: &mut Ui) {
        let layout = egui::Layout::top_down(egui::Align::LEFT);
        ui.allocate_ui_with_layout(Vec2::new(80.0, 150.0), layout, |ui| {
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
                        .spacing(Vec2::new(2.5, 2.5))
                        .show(ui, |ui| {
                            for row in registers.registers.iter() {
                                let format = if *row.1 > u8::MAX as u16 {
                                    format!("0x{:04X}", row.1)
                                } else {
                                    format!("0x{:02X}", row.1)
                                };

                                ui.label(egui::Label::new(&row.0));
                                ui.label(egui::Label::new(format));
                                ui.end_row();
                            }
                            ui.end_row();
                        });
                });
        });
    }
}

pub struct RegisterEditorBuilder<T> {
    cpu: Option<RegisterTable<T>>,
    ppu: Option<RegisterTable<T>>,
    io: Option<RegisterTable<T>>,
}

impl<T> Default for RegisterEditorBuilder<T> {
    fn default() -> Self {
        Self {
            cpu: None,
            ppu: None,
            io: None
        }
    }
}

impl<T: DebugRegister> RegisterEditorBuilder<T> {

    pub fn with_cpu(mut self, cpu: T) -> Self {
        self.cpu = Some(RegisterTable::new(cpu));
        self
    }

    pub fn with_ppu(mut self, ppu: T) -> Self {
        self.ppu = Some(RegisterTable::new(ppu));
        self
    }

    pub fn with_io(mut self, io: T) -> Self {
        self.io = Some(RegisterTable::new(io));
        self
    }

    pub fn build(self) -> RegisterEditor<T> {
        RegisterEditor {
            cpu: self.cpu.unwrap(),
            ppu: self.ppu.unwrap(),
            io: self.io.unwrap(),
        }
    }
}
