use eframe::egui::CtxRef;
use eframe::epi::*;
use egui::Vec2;
use gb_dbg::app::Debugger;
use gb_dbg::flow_control::FlowController;
use gb_dbg::memory::{GBMemoryEditor, MemoryEditorBuilder};

pub struct DebuggerApp {
    pub debugger: Debugger<Vec<u8>>,
}

impl App for DebuggerApp {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        frame.set_window_size(Vec2::new(800.0, 600.0));
        self.debugger.draw(&ctx);
    }

    fn name(&self) -> &str {
        "Debugger"
    }
}

fn main() {
    let mem = vec![0xFFu8; u16::MAX as usize];
    let gbm_mem = MemoryEditorBuilder::new(|mem, address| *mem.get(address).unwrap(), mem)
        .with_write_function(|mem, address, value| {
            mem[address] = value;
            println!("Write!")
        })
        .with_address_range("VRam", 0..0xFF + 1)
        .with_address_range("Ram", 0xFF..0xFFF)
        .build();
    let dgb_app = DebuggerApp {
        debugger: Debugger::new(gbm_mem, FlowController),
    };
    eframe::run_native(Box::new(dgb_app), eframe::NativeOptions::default())
}
