mod memory;
mod registers;

use crate::memory::Memory;
use crate::registers::Registers;
use eframe::egui::CtxRef;
use eframe::epi::*;
use egui::Vec2;
use gb_dbg::dbg_interfaces::{
    DebugOperations, MemoryDebugOperations, RegisterDebugOperations, RegisterMap, RegisterValue,
};
use gb_dbg::debugger::{Debugger, DebuggerBuilder};
use gb_dbg::run_duration::RunDuration;
use std::ops::ControlFlow;

pub struct DebuggerApp {
    pub debugger: Debugger<Memory>,
    pub memory: Memory,
    pub register: Registers,
}

impl App for DebuggerApp {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        self.debugger.draw(ctx, &mut self.memory, &self.register);

        if let Some(flow) = self.debugger.flow_status() {
            match flow {
                ControlFlow::Continue(x) => match x {
                    RunDuration::Step => self.register.pc += 1,
                    RunDuration::RunFrame => self.register.pc += 2,
                    RunDuration::RunSecond => self.register.pc += 3,
                },
                ControlFlow::Break(_) => self.register.pc = 1,
            };
        }
    }

    fn name(&self) -> &str {
        "Debugger"
    }
}

fn main() {
    let dbg: Debugger<Memory> = DebuggerBuilder::new().build();
    let dgb_app = DebuggerApp {
        debugger: dbg,
        memory: Default::default(),
        register: Default::default(),
    };
    let options = NativeOptions {
        resizable: false,
        initial_window_size: Some(Vec2::new(1000.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(Box::new(dgb_app), options)
}
