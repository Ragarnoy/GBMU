mod breakpoints;
pub mod disassembler;
pub mod flow_control;
pub mod memory;
pub mod options;
pub mod registers;
mod status_bar;

use crate::dbg_interfaces::{CpuRegs, DebugOperations};
use crate::debugger::breakpoints::BreakpointEditor;
use crate::debugger::disassembler::DisassemblyViewer;
use crate::debugger::flow_control::FlowController;
use crate::debugger::memory::MemoryViewer;
use crate::debugger::options::DebuggerOptions;
use crate::debugger::registers::RegisterEditor;
use crate::debugger::status_bar::StatusBar;
use crate::until::Until;
use egui::{vec2, Color32, CtxRef};
use std::ops::ControlFlow;

pub struct Debugger<MEM> {
    memory_editor: MemoryViewer<MEM>,
    register_editor: RegisterEditor,
    flow_controller: FlowController,
    disassembler: DisassemblyViewer,
    breakpoint_editor: BreakpointEditor,
    status_bar: StatusBar,
    flow_status: Option<ControlFlow<Until>>,
}

impl<BUS: DebugOperations> Debugger<BUS> {
    pub fn draw(&mut self, ctx: &CtxRef, mut memory: &mut BUS) {
        // ctx.set_debug_on_hover(true);

        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame {
                margin: vec2(8., 8.),
                fill: Color32::from_gray(20),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let style = ui.style_mut();
                style.spacing.button_padding = vec2(16., 4.);

                ui.horizontal(|ui| {
                    if ui.button("Reset").clicked() {
                        log::debug!("clicked on reset");
                    }
                    ui.separator();
                    self.flow_status = self.flow_controller.draw(ui);
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.status_bar.draw(ui, memory);
        });

        self.disassembler
            .may_update_cache(memory.cpu_get(CpuRegs::PC).into(), memory);

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(530.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.disassembler.draw(ui);
                    ui.separator();
                    self.memory_editor.draw(ui, &mut memory);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.register_editor.draw(ui, memory);
            self.breakpoint_editor.draw(ui, memory)
        });
    }

    pub fn flow_status(&mut self) -> Option<ControlFlow<Until>> {
        self.flow_status.take()
    }

    pub fn updated_flow_status(&mut self, memory: &BUS) -> Option<ControlFlow<Until>> {
        if self.breakpoint_editor.are_breakpoints_triggered(memory) {
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

    pub fn build<MEM: DebugOperations>(self) -> Debugger<MEM> {
        Debugger {
            memory_editor: MemoryViewer::new(
                self.options.clone().unwrap_or_default().address_ranges,
            ),
            register_editor: RegisterEditor,
            flow_controller: FlowController,
            disassembler: DisassemblyViewer::default(),
            breakpoint_editor: BreakpointEditor::new(self.options.unwrap_or_default().breakpoints),
            status_bar: StatusBar,
            flow_status: None,
        }
    }
}
