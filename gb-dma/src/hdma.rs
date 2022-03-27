use gb_bus::{Address, Bus, Error, FileOperation, IORegArea, Lock};
use gb_clock::{Tick, Ticker};
use gb_cpu::cpu::Cpu;
use gb_ppu::{drawing, Ppu};

#[derive(PartialEq)]
pub enum HdmaMode {
    Gdma,
    Hdma,
}

#[derive(Default)]
pub struct Hdma {
    src: u16,
    dest: u16,
    active: bool,
    data_chunks_len: u8,
    current_chunk_len: u8,
    last_ppu_mode: Option<drawing::Mode>,
    mode: Option<HdmaMode>,
}

impl Hdma {
    const DEST_STARTING_ADDR: u16 = 0x8000;
    const DATA_CHUNK_SIZE: u8 = 0x10;
    const MAX_DATA_CHUNKS_LEN: u8 = 0x7F;
    const HDMA_MODE_BIT: u8 = 0x80;
    const BYTES_PER_CYCLE: u8 = 2;

    pub fn new_data_chunk(&mut self) {
        self.current_chunk_len = Self::DATA_CHUNK_SIZE;
    }

    fn data_transfer(&mut self, adr_bus: &mut dyn Bus<u8>) {
        let v = adr_bus
            .read(self.src, Some(Lock::Dma))
            .expect("memory unavailable during HDMA");
        if adr_bus.write(self.dest, v, Some(Lock::Dma)).is_err() {
            log::error!(
                "failed to write data '{:x}' at '{:x}' during HDMA",
                v,
                self.dest
            );
        }
        self.src += 1;
        self.dest += 1;
    }

    // Method used before each machine cycle to check hdma status
    // For reference [General Purpose DMA](https://gbdev.io/pandocs/CGB_Registers.html#bit-7--0---general-purpose-dma)
    pub fn check_hdma_state(&mut self, mut cpu: &mut Cpu, ppu: &Ppu) {
        if self.active {
            cpu.halted_dma = match self.mode {
                Some(HdmaMode::Gdma) => {
                    if self.current_chunk_len == 0 {
                        self.new_data_chunk();
                    }
                    true
                }
                Some(HdmaMode::Hdma) => {
                    let current_ppu_mode = ppu.lcd_reg.borrow().stat.mode().unwrap();
                    let is_new_hblank = current_ppu_mode == drawing::Mode::HBlank
                        && Some(current_ppu_mode) != self.last_ppu_mode;
                    if self.current_chunk_len == 0 && is_new_hblank {
                        self.new_data_chunk();
                    }
                    self.last_ppu_mode = Some(current_ppu_mode);

                    current_ppu_mode == drawing::Mode::HBlank && self.current_chunk_len > 0
                }
                None => false,
            }
        } else {
            cpu.halted_dma = false;
        }
    }
}

impl<A> FileOperation<A, IORegArea> for Hdma
where
    u16: From<A>,
    A: Address<IORegArea>,
{
    fn read(&self, addr: A) -> Result<u8, gb_bus::Error> {
        match addr.area_type() {
            IORegArea::Hdma1 => Ok(self.src.to_be_bytes()[1]),
            IORegArea::Hdma2 => Ok(self.src as u8),
            IORegArea::Hdma3 => Ok(self.dest.to_be_bytes()[1]),
            IORegArea::Hdma4 => Ok(self.dest as u8),
            IORegArea::Hdma5 => Ok(self.data_chunks_len
                | if self.active {
                    0x00
                } else {
                    Self::HDMA_MODE_BIT
                }),
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
    fn write(&mut self, v: u8, addr: A) -> Result<(), gb_bus::Error> {
        match addr.area_type() {
            IORegArea::Hdma1 => {
                self.src = ((v as u16) << 8) | (self.src & 0x00FF);
                Ok(())
            }
            IORegArea::Hdma2 => {
                self.src = (self.src & 0xFF00) | ((v & 0xF0) as u16);
                Ok(())
            }
            IORegArea::Hdma3 => {
                self.dest =
                    Self::DEST_STARTING_ADDR | (((v & 0x1f) as u16) << 8) | (self.dest & 0xFF);
                Ok(())
            }
            IORegArea::Hdma4 => {
                self.dest = (self.dest & 0xFF00) | ((v & 0xF0) as u16);
                Ok(())
            }
            IORegArea::Hdma5 => {
                if self.active && self.mode == Some(HdmaMode::Hdma) {
                    if v & Self::HDMA_MODE_BIT == 0 {
                        self.active = false;
                    };
                    return Ok(());
                }
                self.active = true;
                self.data_chunks_len = v & Self::MAX_DATA_CHUNKS_LEN;
                self.current_chunk_len = Self::DATA_CHUNK_SIZE;
                self.mode = match v & Self::HDMA_MODE_BIT {
                    0 => Some(HdmaMode::Gdma),
                    _ => Some(HdmaMode::Hdma),
                };
                Ok(())
            }
            _ => Err(Error::SegmentationFault(addr.into())),
        }
    }
}

impl Ticker for Hdma {
    fn cycle_count(&self) -> Tick {
        Tick::MCycle
    }

    fn tick(&mut self, adr_bus: &mut dyn Bus<u8>) {
        if !self.active || self.current_chunk_len == 0 {
            return;
        }
        for _ in 0..Self::BYTES_PER_CYCLE {
            self.data_transfer(adr_bus);

            self.current_chunk_len -= 1;
            if self.current_chunk_len == 0 {
                if self.data_chunks_len == 0 {
                    self.active = false;
                    self.data_chunks_len = Self::MAX_DATA_CHUNKS_LEN;
                } else {
                    self.data_chunks_len -= 1;
                }
                return;
            }
        }
    }
}
