mod memory;
mod registers;

use crate::memory::Memory;
use crate::registers::Registers;
use eframe::egui::CtxRef;
use eframe::epi::*;
use egui::Vec2;
use gb_dbg::debugger::{Debugger, DebuggerBuilder};

pub struct DebuggerApp {
    pub debugger: Debugger<Memory>,
    pub memory: Memory,
    pub register: Registers,
}

impl App for DebuggerApp {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        frame.set_window_size(Vec2::new(1000.0, 600.0));
        self.debugger.draw(ctx, &mut self.memory, &self.register);
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
        register: Default::default()
    };
    eframe::run_native(Box::new(dgb_app), eframe::NativeOptions::default())
}
