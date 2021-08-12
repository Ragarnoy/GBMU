use egui::{CtxRef};
use crate::memory::GBMemoryEditor;
use crate::flow_control::FlowController;

pub struct DebugApp<T> {
    memory_editor: GBMemoryEditor<T>,
    flow_controller: FlowController,
}

impl<T> DebugApp<T> {

    pub fn new(memory_editor: GBMemoryEditor<T>, flow_controller: FlowController) -> Self {
        Self {
            memory_editor,
            flow_controller
        }
    }

    pub fn draw(&mut self, ctx: &CtxRef) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.flow_controller.draw(ui);
        });
        egui::SidePanel::left("left_panel").resizable(false).default_width(200.0).show(ctx, |ui| {
            self.memory_editor.draw(ui);
        });
    }
}
