use egui::Ui;
use egui_memory_editor::option_data::MemoryEditorOptions;
use egui_memory_editor::{MemoryEditor, ReadFunction, WriteFunction};
use std::ops::Range;

pub struct GBMemoryEditor<T> {
    memory: T,
    memory_editor: MemoryEditor<T>,
}

impl<T> GBMemoryEditor<T> {
    pub fn draw(&mut self, ui: &mut Ui) {
        self.memory_editor
            .draw_editor_contents(ui, &mut self.memory);
    }
}

pub struct MemoryEditorBuilder<'d, T> {
    read_func: ReadFunction<T>,
    write_func: Option<WriteFunction<T>>,
    address_ranges: [(&'d str, Range<usize>); 3],
    memory: T,
}

impl<T> MemoryEditorBuilder<'_, T> {
    pub fn new(read_func: ReadFunction<T>, memory: T) -> Self {
        let arr: [(&str, Range<usize>); 3] = [
            ("WRam", 0..0xAA),
            ("VRam", 0xAA..0xBB),
            ("Placeholder", 0xBB..0xCC),
        ];
        Self {
            read_func,
            write_func: None,
            address_ranges: arr,
            memory,
        }
    }

    pub fn with_write_function(mut self, write_func: WriteFunction<T>) -> Self {
        self.write_func = Some(write_func);
        self
    }

    pub fn build(self) -> GBMemoryEditor<T> {
        let mut mem_options = MemoryEditorOptions::default();
        mem_options.is_resizable_column = false;
        mem_options.is_options_collapsed = true;
        let mem_edit = MemoryEditor::new(self.read_func)
            .with_options(mem_options)
            .with_address_range(self.address_ranges[0].0, self.address_ranges[0].1.clone())
            .with_address_range(self.address_ranges[1].0, self.address_ranges[1].1.clone())
            .with_address_range(self.address_ranges[2].0, self.address_ranges[2].1.clone());

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
