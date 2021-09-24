use egui::{Color32, Label, Ui, Vec2};
use crate::dbg_interfaces::MemoryDebugOperations;
use gb_roms::opcode::OpcodeGenerator;

pub struct DisassemblyViewer;

impl DisassemblyViewer {
    pub fn draw<MEM: MemoryDebugOperations>(&self, ui: &mut Ui, pc: u16, memory: &MEM) {
        ui.label(Label::new("Disassembler").text_color(Color32::WHITE));
        egui::CollapsingHeader::new("🛠 Options")
            .id_source(55)
            .default_open(false)
            .show(ui, |ui| ui.label("Hello"));
        ui.separator();
        ui.vertical(|ui| {
            egui::Grid::new("dissas_".to_owned())
                .striped(true)
                .spacing(Vec2::new(100.0, 2.5))
                .show(ui, |ui| {
                    ui.label(egui::Label::new("Address").text_color(Color32::WHITE));
                    ui.label(egui::Label::new("Instruction").text_color(Color32::WHITE));
                    ui.label(egui::Label::new("Data").text_color(Color32::WHITE));
                    ui.end_row();
                    let byte_it = ByteIterator::new(pc, memory);
                    let generator = OpcodeGenerator::from(byte_it);
                    for (row_number, row) in generator.take(8).enumerate() {
                        ui.label(egui::Label::new(format!("0x{:04X}", pc + row_number as u16 + 1)));
                        ui.label(egui::Label::new(format!("{}", row.unwrap())));
                        ui.label(egui::Label::new("0x00000000"));
                        ui.end_row();
                    }
                    ui.end_row();
                });
        });
    }
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
