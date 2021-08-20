use egui::{Color32, Label, Ui};
use egui_memory_editor::option_data::MemoryEditorOptions;
use egui_memory_editor::{MemoryEditor, ReadFunction, WriteFunction};
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

pub struct MemoryEditorBuilder<'name, T> {
    read_func: ReadFunction<T>,
    write_func: Option<WriteFunction<T>>,
    address_ranges: Vec<(&'name str, Range<usize>)>,
    memory: T,
}

impl<'name, T> MemoryEditorBuilder<'name, T> {
    pub fn new(read_func: ReadFunction<T>, memory: T) -> Self {
        Self {
            read_func,
            write_func: None,
            address_ranges: Vec::with_capacity(6),
            memory,
        }
    }

    pub fn with_address_range(mut self, range_name: &'name str, range: Range<usize>) -> Self {
        self.address_ranges.push((range_name, range));
        self
    }

    pub fn with_write_function(mut self, write_func: WriteFunction<T>) -> Self {
        self.write_func = Some(write_func);
        self
    }

    pub fn build(self) -> GBMemoryEditor<T> {
        let mut mem_options = MemoryEditorOptions::default();
        mem_options.is_resizable_column = false;
        mem_options.is_options_collapsed = true;
        let mut mem_edit = MemoryEditor::new(self.read_func).with_options(mem_options);
        for (range_name, range) in self.address_ranges {
            mem_edit = mem_edit.with_address_range(range_name, range);
        }

        if self.write_func.is_some() {
            GBMemoryEditor {
                memory: self.memory,
                memory_editor: mem_edit.with_write_function(self.write_func.unwrap()),
            }
        } else {
            GBMemoryEditor {
                memory: self.memory,
                memory_editor: mem_edit,
            }
        }
    }
}
