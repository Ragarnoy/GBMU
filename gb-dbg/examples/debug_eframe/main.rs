mod game;

use eframe::egui::CtxRef;
use eframe::epi::*;
use egui::Vec2;
use game::Game;
use gb_dbg::debugger::{Debugger, DebuggerBuilder};
use gb_dbg::until::Until;
use gb_dbg::{DEBUGGER_HEIGHT, DEBUGGER_WIDTH};
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
                    Until::Step(n) => self.memory.pc += (n << 1) as u16,
                    Until::Instruction(_opcode) => self.memory.pc += 1 as u16,
                    Until::Frame(n) => self.memory.pc += (n << 2) as u16,
                    Until::Second(n) => self.memory.pc += (n << 3) as u16,
                    Until::Null => self.memory.pc = 0,
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
        initial_window_size: Some(Vec2::from((DEBUGGER_WIDTH, DEBUGGER_HEIGHT))),
        ..Default::default()
    };
    eframe::run_native(Box::new(dgb_app), options)
}
