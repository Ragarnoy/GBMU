mod game;

use eframe::egui::CtxRef;
use eframe::epi::*;
use egui::Vec2;
use game::Game;
use gb_dbg::debugger::{Debugger, DebuggerBuilder};
use gb_dbg::run_duration::RunDuration;
use std::ops::ControlFlow;

pub struct DebuggerApp {
    pub debugger: Debugger<Game>,
    pub memory: Game,
}

impl App for DebuggerApp {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        self.debugger.draw(ctx, &mut self.memory);

        if let Some(flow) = self.debugger.flow_status() {
            match flow {
                ControlFlow::Break(x) => match x {
                    RunDuration::Step => self.memory.pc += 1,
                    RunDuration::RunFrame => self.memory.pc += 2,
                    RunDuration::RunSecond => self.memory.pc += 3,
                },
                ControlFlow::Continue(_) => self.memory.pc = 1,
            };
        }
    }

    fn name(&self) -> &str {
        "Debugger"
    }
}

fn main() {
    let dbg: Debugger<Game> = DebuggerBuilder::new().build();
    let dgb_app = DebuggerApp {
        debugger: dbg,
        memory: Default::default(),
    };
    let options = NativeOptions {
        resizable: false,
        initial_window_size: Some(Vec2::new(1000.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(Box::new(dgb_app), options)
}
