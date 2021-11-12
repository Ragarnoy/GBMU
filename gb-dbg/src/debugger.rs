mod breakpoints;
pub mod disassembler;
pub mod flow_control;
pub mod memory;
mod options;
pub mod registers;

use crate::dbg_interfaces::{CpuRegs, DebugOperations, MemoryDebugOperations};
use crate::debugger::breakpoints::BreakpointEditor;
use crate::debugger::disassembler::DisassemblyViewer;
use crate::debugger::flow_control::FlowController;
use crate::debugger::memory::MemoryViewer;
use crate::debugger::options::DebuggerOptions;
use crate::debugger::registers::RegisterEditor;
use crate::until::Until;
use egui::CtxRef;
use std::ops::ControlFlow;

pub struct Debugger<MEM> {
    memory_editor: MemoryViewer<MEM>,
    register_editor: RegisterEditor,
    flow_controller: FlowController,
    disassembler: DisassemblyViewer,
    breakpoint_editor: BreakpointEditor,
    flow_status: Option<ControlFlow<Until>>,
}

impl<MEM: DebugOperations> Debugger<MEM> {
    pub fn draw(&mut self, ctx: &CtxRef, mut memory: &mut MEM) {
        // ctx.set_debug_on_hover(true);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.flow_status = self.flow_controller.draw(ui);
        });

        self.disassembler
            .may_update_cache(memory.cpu_get(CpuRegs::PC).unwrap().into(), memory);

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(510.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.disassembler.draw(ui);
                    ui.separator();
                    self.memory_editor.draw(ui, &mut memory);
                });
            });

        egui::SidePanel::right("right_panel")
            .resizable(false)
            .default_width(130.0)
            .show(ctx, |ui| {
                self.breakpoint_editor
                    .draw(ui, memory.cpu_get(CpuRegs::PC).unwrap().into())
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.register_editor.draw(ui, memory);
        });
    }

    pub fn flow_status(&mut self) -> Option<ControlFlow<Until>> {
        self.flow_status.take()
    }

    pub fn updated_flow_status(&mut self, memory: &MEM) -> Option<ControlFlow<Until>> {
        if self
            .breakpoint_editor
            .are_breakpoints_trigger(memory.cpu_get(CpuRegs::PC).unwrap().into())
        {
            Some(ControlFlow::Break(Until::Null))
        } else {
            self.flow_status()
        }
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
            disassembler: DisassemblyViewer::default(),
            breakpoint_editor: BreakpointEditor::default(),
            flow_status: None,
        }
    }
}
