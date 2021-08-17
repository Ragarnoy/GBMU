use egui::{CtxRef};
use crate::memory::GBMemoryEditor;
use crate::flow_control::FlowController;
use crate::disassembler::Disassembler;

pub struct Debugger<T> {
    memory_editor: GBMemoryEditor<T>,
    flow_controller: FlowController,
    disassembler: Disassembler,
}

impl<T> Debugger<T> {

    pub fn new(memory_editor: GBMemoryEditor<T>, flow_controller: FlowController, disassembler: Disassembler) -> Self {
        Self {
            memory_editor,
            flow_controller,
            disassembler
        }
    }

    pub fn draw(&mut self, ctx: &CtxRef) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.flow_controller.draw(ui);
        });
        egui::SidePanel::left("left_panel").resizable(false).default_width(200.0).show(ctx, |ui| {
            ui.vertical(|ui| {
                self.disassembler.draw(ui);
                ui.separator();
                self.memory_editor.draw(ui);
            });
        });
        // egui::SidePanel::right("right_panel").resizable(false).show(ctx, |ui| {
        // });
    }
}
