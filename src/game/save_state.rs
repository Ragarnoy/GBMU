use gb_bus::WorkingRam;
use gb_dma::dma::Dma;
use gb_dma::hdma::Hdma;
use gb_ppu::Ppu;
use gb_roms::controllers::{Full, GenericState};
use gb_timer::Timer;

use crate::game::Game;

#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct SaveState {
    pub romname: String,
    pub cpu_regs: gb_cpu::registers::Registers,
    pub cpu_io_regs: gb_cpu::io_registers::IORegisters,
    pub mbcs: GenericState<Full>,
    pub working_ram: WorkingRam,
    pub timer: Timer,
    pub hram: Vec<u8>,
    pub ppu: Ppu,
    pub dma: Dma,
    pub hdma: Hdma,
}

impl From<&Game> for SaveState {
    fn from(context: &Game) -> Self {
        Self {
            romname: context.romname.clone(),
            cpu_regs: context.cpu.registers,
            cpu_io_regs: *context.cpu.io_regs.borrow(),
            mbcs: context.mbc.borrow().save(),
            working_ram: context.wram.borrow().clone(),
            timer: *context.timer.borrow(),
            hram: context.hram.borrow().save(),
            ppu: context.ppu.clone(),
            dma: *context.dma.borrow(),
            hdma: *context.hdma.borrow(),
        }
    }
}
