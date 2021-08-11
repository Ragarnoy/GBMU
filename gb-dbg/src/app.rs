use egui::{CtxRef};
use crate::memory::GBMemoryEditor;

pub struct DebugApp<T> {
    memory_editor: GBMemoryEditor<T>,
}

impl<T> DebugApp<T> {

    pub fn new(memory_editor: GBMemoryEditor<T>) -> Self {
        Self {
            memory_editor
        }
    }

    pub fn draw(&mut self, ctx: &CtxRef) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.button("Step");
                ui.button("Run one frame");
                ui.button("Run one second")
            });
        });
        egui::SidePanel::left("left_panel").resizable(false).default_width(200.0).show(ctx, |ui| {
            self.memory_editor.draw(ui);
        });
    }
}