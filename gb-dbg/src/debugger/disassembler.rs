use crate::dbg_interfaces::MemoryDebugOperations;
use egui::{Color32, Label, Ui, Vec2};
use gb_roms::opcode::{error::Error, list::Opcode, OpcodeGenerator};

#[derive(Default)]
pub struct DisassemblyViewer {
    cache: Vec<Result<(Opcode, Vec<u8>), Error>>,
    cache_pc_valid_range: Option<(u16, u16)>,
}

impl DisassemblyViewer {
    fn update_cache<MEM: MemoryDebugOperations>(&mut self, pc: u16, memory: &MEM) {
        log::debug!("update opcode cache");
        let byte_it = ByteIterator::new(pc, memory);
        let generator = OpcodeGenerator::from(byte_it);
        self.cache = generator.take(8).collect::<Vec<Result<_, Error>>>();
        self.cache_pc_valid_range = Some((pc, opcode_len(&self.cache[0]) + pc));
    }

    pub fn may_update_cache<MEM: MemoryDebugOperations>(&mut self, pc: u16, memory: &MEM) {
        if let Some(range) = self.cache_pc_valid_range {
            if pc < range.0 || pc >= range.1 {
                self.update_cache(pc, memory)
            }
        } else {
            log::debug!("initialise opcode cache");
            self.update_cache(pc, memory)
        }
    }

    pub fn draw(&self, ui: &mut Ui) {
        ui.label(Label::new("Disassembler").text_color(Color32::WHITE));
        egui::CollapsingHeader::new("🛠 Options")
            .id_source(55)
            .default_open(false)
            .show(ui, |ui| ui.label("Hello"));
        ui.separator();
        ui.vertical(|ui| {
            egui::Grid::new("dissas_".to_owned())
                .striped(true)
                .spacing(Vec2::new(150.0, 2.5))
                .show(ui, |ui| {
                    ui.label(egui::Label::new("Address").text_color(Color32::WHITE));
                    ui.label(egui::Label::new("Instruction").text_color(Color32::WHITE));
                    ui.label(egui::Label::new("Data").text_color(Color32::WHITE));
                    ui.end_row();
                    let mut pc = self.cache_pc_valid_range.unwrap_or((0, 0)).0;
                    for row in self.cache.iter().take(8) {
                        ui.label(egui::Label::new(format!("0x{:04X}", pc)));
                        pc += opcode_len(row);
                        let opcode = row
                            .as_ref()
                            .map_or("??".to_string(), |(opc, _)| opc.to_string());
                        let bytes = row.as_ref().map_or_else(
                            |e| match e {
                                Error::InvalidRegisterValue(v)
                                | Error::InvalideOpcode(v)
                                | Error::UnknownOpcode(v) => v.to_string(),
                            },
                            |(_, bytes)| {
                                bytes.iter().fold(String::with_capacity(8), |acc, &s| {
                                    acc + format!("0x{:02X} ", s).as_str()
                                })
                            },
                        );
                        ui.label(egui::Label::new(opcode));
                        ui.label(egui::Label::new(bytes));
                        ui.end_row();
                    }
                    ui.end_row();
                });
        });
    }
}

fn opcode_len(opc: &Result<(Opcode, Vec<u8>), Error>) -> u16 {
    opc.as_ref().map_or(1, |(_, bytes)| bytes.len() as u16)
}

struct ByteIterator<'a, MEM: MemoryDebugOperations> {
    start_address: u16,
    address_offset: u16,
    memory: &'a MEM,
}

impl<'a, MEM: MemoryDebugOperations> ByteIterator<'a, MEM> {
    fn new(start_address: u16, memory: &'a MEM) -> Self {
        Self {
            start_address,
            address_offset: 0,
            memory,
        }
    }
}

impl<'a, MEM: MemoryDebugOperations> Iterator for ByteIterator<'a, MEM> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let address: u16 = self.start_address + self.address_offset;
        self.address_offset += 1;
        Some(self.memory.read(address))
    }
}
