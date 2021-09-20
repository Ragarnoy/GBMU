pub mod disassembler;
pub mod flow_control;
pub mod memory;
pub mod registers;
mod options;

use crate::dbg_interfaces::{RegisterDebugOperations, MemoryDebugOperations};
use crate::debugger::disassembler::Disassembler;
use crate::debugger::flow_control::FlowController;
use crate::debugger::memory::MemoryViewer;
use crate::debugger::registers::RegisterEditor;
use egui::{Color32, CtxRef, Label};
use crate::debugger::options::DebuggerOptions;

pub struct Debugger<MEM, REG> {
    memory_editor: MemoryViewer<MEM>,
    register_editor: RegisterEditor<REG>,
    flow_controller: FlowController,
    disassembler: Disassembler,
}

impl<MEM: MemoryDebugOperations, REG: RegisterDebugOperations> Debugger<MEM, REG> {
    pub fn new(
        memory_editor: MemoryViewer<MEM>,
        register_editor: RegisterEditor<REG>,
        flow_controller: FlowController,
        disassembler: Disassembler,
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

pub struct DebuggerBuilder<MEM, REG> {
    memory_interface: MEM,
    register_interface: REG,
    options: Option<DebuggerOptions>,
}

impl<MEM, REG> DebuggerBuilder<MEM, REG>
where
    MEM: MemoryDebugOperations,
    REG: RegisterDebugOperations,
{
    pub fn new(memory_interface: MEM, register_interface: REG) -> Self<MEM, REG> {
        Self {
            memory_interface,
            register_interface,
            options: None,
        }
    }

    pub fn with_options(mut self, options: DebuggerOptions) -> Self {
        self.options = Some(options);
        self
    }
}
