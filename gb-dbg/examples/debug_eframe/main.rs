mod memory;
mod registers;

use crate::memory::Memory;
use crate::registers::Registers;
use eframe::egui::CtxRef;
use eframe::epi::*;
use egui::Vec2;
use gb_dbg::dbg_interfaces::RW;
use gb_dbg::debugger::disassembler::Disassembler;
use gb_dbg::debugger::flow_control::FlowController;
use gb_dbg::debugger::memory::MemoryEditorBuilder;
use gb_dbg::debugger::registers::RegisterEditorBuilder;
use gb_dbg::debugger::Debugger;

pub struct DebuggerApp {
    pub debugger: Debugger<Memory, Registers>,
}

impl RW for Memory {
    fn read(&self, index: usize) -> u8 {
        *self.memory.get(index).unwrap()
    }

    fn write(&mut self, _index: usize, _value: u8) {
        self.memory[_index] = _value
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
        .with_address_range("VRam", 0..0xFF + 1)
        .with_address_range("Ram", 0xFF..0xFFF)
        .build();
    let rega = Registers::default();
    let regb = Registers::default();
    let regc = Registers::default();
    let regs = RegisterEditorBuilder::default()
        .with_cpu(rega)
        .with_ppu(regb)
        .with_io(regc)
        .build();
    let dgb_app = DebuggerApp {
        debugger: Debugger::new(gbm_mem, regs, FlowController, Disassembler),
    };
    eframe::run_native(Box::new(dgb_app), eframe::NativeOptions::default())
}
