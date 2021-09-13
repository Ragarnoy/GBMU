pub mod disassembler;
pub mod flow_control;
pub mod memory;
pub mod registers;

use crate::dbg_interfaces::{DebugRegister, RW};
use crate::debugger::disassembler::DisassemblyEditor;
use crate::debugger::flow_control::FlowController;
use crate::debugger::memory::GBMemoryEditor;
use crate::debugger::registers::RegisterEditor;
use egui::{Color32, CtxRef, Label};

pub struct Debugger<T, R> {
    memory_editor: GBMemoryEditor<T>,
    register_editor: RegisterEditor<R>,
    flow_controller: FlowController,
    disassembler: DisassemblyEditor,
}

impl<T: RW, R: DebugRegister> Debugger<T, R> {
    pub fn new(
        memory_editor: GBMemoryEditor<T>,
        register_editor: RegisterEditor<R>,
        flow_controller: FlowController,
        disassembler: DisassemblyEditor,
    ) -> Self {
        Self {
            memory_editor,
            register_editor,
            flow_controller,
            disassembler,
        }
    }

    pub fn draw(&mut self, ctx: &CtxRef) {
        // ctx.set_debug_on_hover(true);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.flow_controller.draw(ui);
        });
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(545.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.disassembler.draw(ui);
                    ui.separator();
                    self.memory_editor.draw(ui);
                });
            });
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .default_width(170.0)
            .show(ctx, |ui| {
                ui.label(Label::new("Breakpoints").text_color(Color32::WHITE));
                egui::CollapsingHeader::new("🛠 Options")
                    .id_source(55)
                    .default_open(false)
                    .show(ui, |ui| ui.label("Hello"));
                ui.separator();
                ui.columns(2, |columns| {
                    columns[0].label("Enable");
                    columns[1].label("Address");
                })
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.register_editor.draw(ui);
        });
    }
}
