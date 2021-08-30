pub mod disassembler;
pub mod flow_control;
pub mod memory;

use egui::{Color32, CtxRef, Label};
use crate::debugger::memory::GBMemoryEditor;
use crate::debugger::flow_control::FlowController;
use crate::debugger::disassembler::Disassembler;
use crate::dbg_interfaces::RW;

pub struct Debugger<T: RW> {
    memory_editor: GBMemoryEditor<T>,
    flow_controller: FlowController,
    disassembler: Disassembler,
}

impl<T: RW> Debugger<T> {
    pub fn new(
        memory_editor: GBMemoryEditor<T>,
        flow_controller: FlowController,
        disassembler: Disassembler,
    ) -> Self {
        Self {
            memory_editor,
            flow_controller,
            disassembler,
        }
    }

    pub fn draw(&mut self, ctx: &CtxRef) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.flow_controller.draw(ui);
        });
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.disassembler.draw(ui);
                    ui.separator();
                    self.memory_editor.draw(ui);
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(Label::new("Register Editors").text_color(Color32::WHITE));
            ui.separator();
        });
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .min_width(150.0)
            .show(ctx, |ui| {
                ui.label(Label::new("Breakpoints").text_color(Color32::WHITE));
                ui.separator();
                ui.columns(2, |columns| {
                    columns[0].label("");
                    columns[1].label("Address");
                })
            });
    }
}
