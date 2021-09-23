pub mod disassembler;
pub mod flow_control;
pub mod memory;
mod options;
pub mod registers;

use crate::dbg_interfaces::{MemoryDebugOperations, RegisterDebugOperations};
use crate::debugger::disassembler::DisassemblyViewer;
use crate::debugger::flow_control::FlowController;
use crate::debugger::memory::MemoryViewer;
use crate::debugger::options::DebuggerOptions;
use crate::debugger::registers::RegisterEditor;
use egui::{Color32, CtxRef, Label};

pub struct Debugger<MEM> {
    memory_editor: MemoryViewer<MEM>,
    register_editor: RegisterEditor,
    flow_controller: FlowController,
    disassembler: DisassemblyViewer,
}

impl<MEM: MemoryDebugOperations> Debugger<MEM> {
    pub fn draw<REG: RegisterDebugOperations>(
        &mut self,
        ctx: &CtxRef,
        mut memory: &mut MEM,
        registers: &REG,
    ) {
        // ctx.set_debug_on_hover(true);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.flow_controller.draw(ui);
        });
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(545.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.disassembler.draw(ui, registers.cpu_get("PC").unwrap().into());
                    ui.separator();
                    self.memory_editor.draw(ui, &mut memory);
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
            self.register_editor.draw(ui, registers);
        });
    }
}

#[derive(Default)]
pub struct DebuggerBuilder {
    options: Option<DebuggerOptions>,
}

impl DebuggerBuilder {
    pub fn new() -> Self {
        Self { options: None }
    }

    pub fn with_options(mut self, options: DebuggerOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub fn build<MEM: MemoryDebugOperations>(self) -> Debugger<MEM> {
        Debugger {
            memory_editor: MemoryViewer::new(self.options.unwrap_or_default().address_ranges),
            register_editor: RegisterEditor,
            flow_controller: FlowController,
            disassembler: DisassemblyViewer,
        }
    }
}