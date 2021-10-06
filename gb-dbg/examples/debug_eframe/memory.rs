use gb_dbg::dbg_interfaces::MemoryDebugOperations;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl MemoryDebugOperations for Memory {
    fn read(&self, index: u16) -> u8 {
        *self.memory.get(index as usize).unwrap()
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: vec![0xFFu8; u16::MAX as usize],
        }
    }
}
