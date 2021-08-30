use crate::interfaces::RW;
use egui::{Color32, Label, Ui};
use egui_memory_editor::option_data::MemoryEditorOptions;
use egui_memory_editor::MemoryEditor;
use std::ops::Range;

pub struct GBMemoryEditor<T> {
    memory: T,
    memory_editor: MemoryEditor<T>,
}

impl<T> GBMemoryEditor<T> {
    pub fn draw(&mut self, ui: &mut Ui) {
        ui.label(Label::new("Memory Editor").text_color(Color32::WHITE));
        self.memory_editor
            .draw_editor_contents(ui, &mut self.memory);
    }
}

pub struct MemoryEditorBuilder<T: RW> {
    address_ranges: Vec<(String, Range<usize>)>,
    memory: T,
}

impl<T: RW> MemoryEditorBuilder<T> {
    pub fn new(memory: T) -> Self {
        Self {
            address_ranges: Vec::with_capacity(6),
            memory,
        }
    }

    pub fn with_address_range(mut self, range_name: &str, range: Range<usize>) -> Self {
        self.address_ranges.push((range_name.to_owned(), range));
        self
    }

    pub fn build(self) -> GBMemoryEditor<T> {
        let mut mem_options = MemoryEditorOptions::default();
        mem_options.is_resizable_column = false;
        mem_options.is_options_collapsed = true;
        let mut mem_edit = MemoryEditor::new(|mem, address| <T as RW>::read(mem, address))
            .with_options(mem_options);
        for (range_name, range) in self.address_ranges {
            mem_edit = mem_edit.with_address_range(range_name, range);
        }

        GBMemoryEditor {
            memory: self.memory,
            memory_editor: mem_edit.with_write_function(|mut mem, address, value| {
                <T as RW>::write(&mut mem, address, value)
            }),
        }
    }
}
