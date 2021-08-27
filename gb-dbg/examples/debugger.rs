use eframe::egui::CtxRef;
use eframe::epi::*;
use egui::Vec2;
use gb_dbg::debugger::disassembler::Disassembler;
use gb_dbg::debugger::flow_control::FlowController;
use gb_dbg::debugger::memory::MemoryEditorBuilder;
use gb_dbg::debugger::Debugger;
use gb_dbg::rw_interface::DebugRW;

pub struct DebuggerApp {
    pub debugger: Debugger<Memory>,
}

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: vec![0xFFu8; u16::MAX as usize]
        }
    }
}

impl DebugRW for Memory {
    type RegisterIter = todo!();

    fn read(&self, index: usize) -> u8 {
        *self.memory.get(index).unwrap()
    }

    fn write(&mut self, index: usize, value: u8) {
        self.memory[index] = value
    }

    fn register_iter(&self) -> Self::RegisterIter {
        todo!()
    }
}

impl App for DebuggerApp {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
        frame.set_window_size(Vec2::new(1000.0, 600.0));
        self.debugger.draw(ctx);
    }

    fn name(&self) -> &str {
        "Debugger"
    }
}

fn main() {
    let mem = Default::default();
    let gbm_mem = MemoryEditorBuilder::new(mem)
        .with_write_function(|mem, address, value| {
            mem[address] = value;
            println!("Write!")
        })
        .with_address_range("VRam", 0..0xFF + 1)
        .with_address_range("Ram", 0xFF..0xFFF)
        .build();
    let dgb_app = DebuggerApp {
        debugger: Debugger::new(gbm_mem, FlowController, Disassembler),
    };
    eframe::run_native(Box::new(dgb_app), eframe::NativeOptions::default())
}
