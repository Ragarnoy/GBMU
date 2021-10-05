pub mod disassembler;
pub mod flow_control;
pub mod memory;
mod options;
pub mod registers;
mod breakpoints;

use crate::dbg_interfaces::{MemoryDebugOperations, RegisterDebugOperations};
use crate::debugger::disassembler::DisassemblyViewer;
use crate::debugger::flow_control::FlowController;
use crate::debugger::memory::MemoryViewer;
use crate::debugger::options::DebuggerOptions;
use crate::debugger::registers::RegisterEditor;
use egui::{CtxRef};
use crate::debugger::breakpoints::BreakpointEditor;

pub struct Debugger<MEM> {
    memory_editor: MemoryViewer<MEM>,
    register_editor: RegisterEditor,
    flow_controller: FlowController,
    disassembler: DisassemblyViewer,
    breakpoint_editor: BreakpointEditor,
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
            .default_width(510.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.disassembler
                        .draw(ui, registers.cpu_get("PC").unwrap().into(), memory);
                    ui.separator();
                    self.memory_editor.draw(ui, &mut memory);
                });
            });
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .default_width(170.0)
            .show(ctx, |ui| {
                self.breakpoint_editor.draw(ui, memory);
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
            breakpoint_editor: BreakpointEditor::default(),
        }
    }
}
