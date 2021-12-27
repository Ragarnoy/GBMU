use crate::dbg_interfaces::MemoryDebugOperations;
use egui::{Color32, Ui};
use egui_memory_editor::option_data::MemoryEditorOptions;
use egui_memory_editor::MemoryEditor;
use std::ops::Range;

pub struct MemoryViewer<MEM> {
    memory_editor: MemoryEditor<MEM>,
}

impl<MEM: MemoryDebugOperations> MemoryViewer<MEM> {
    pub fn new(address_ranges: Vec<(&'static str, Range<u16>)>) -> Self {
        let mut mem_options = MemoryEditorOptions::default();
        mem_options.address_text_colour = Color32::from_rgb(191, 64, 191);
        mem_options.is_resizable_column = false;
        mem_options.is_options_collapsed = false;
        let mut mem_edit = MemoryEditor::new(|mem, address| {
            <MEM as MemoryDebugOperations>::read(mem, address as u16)
        })
        .with_options(mem_options);
        for (range_name, range) in address_ranges {
            mem_edit = mem_edit
                .with_address_range(range_name, range.start as usize..range.end as usize + 1);
        }

        Self {
            memory_editor: mem_edit,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, memory: &mut MEM) {
        ui.colored_label(Color32::LIGHT_BLUE, "Memory Editor");
        self.memory_editor.draw_editor_contents(ui, memory);
    }
}
