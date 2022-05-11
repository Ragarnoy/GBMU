mod game;

use eframe::egui::Context;
use eframe::epi::*;
use egui::Vec2;
use game::Game;
use gb_dbg::debugger::{Debugger, DebuggerBuilder};
use gb_dbg::until::Until;
use gb_dbg::{DEBUGGER_HEIGHT, DEBUGGER_WIDTH};
use std::ops::ControlFlow;

pub struct DebuggerApp {
    pub debugger: Debugger,
    pub memory: Game,
}

impl App for DebuggerApp {
    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        self.debugger.draw(ctx, &mut self.memory, None);

        if let Some(flow) = self.debugger.flow_status() {
            match flow {
                ControlFlow::Break(x) => match x {
                    Until::Cycle(n) => self.memory.pc += (n << 1) as u16,
                    Until::Step(_n) => self.memory.pc += 1_u16,
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
    let dbg: Debugger = DebuggerBuilder::new().build();
    let dgb_app = DebuggerApp {
        debugger: dbg,
        memory: Default::default(),
    };
    let options = NativeOptions {
        resizable: false,
        initial_window_size: Some(Vec2::from((DEBUGGER_WIDTH as f32, DEBUGGER_HEIGHT as f32))),
        ..Default::default()
    };
    eframe::run_native(Box::new(dgb_app), options)
}
