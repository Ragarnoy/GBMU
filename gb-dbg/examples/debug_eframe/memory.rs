use gb_dbg::dbg_interfaces::MemoryDebugOperations;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl MemoryDebugOperations for Memory {
    fn read(&self, index: usize) -> u8 {
        *self.memory.get(index).unwrap()
    }

    fn write(&mut self, _index: usize, _value: u8) {
        self.memory[_index] = _value
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: vec![0xFFu8; u16::MAX as usize],
        }
    }
}
