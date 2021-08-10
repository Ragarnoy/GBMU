pub struct RomOnly {
    data: Vec<u8>
}

impl RomOnly {
    pub fn new(data: Vec<u8>) -> Self {
        RomOnly {
            data
        }
    }

    pub fn read(memory: &Vec<u8>, address: usize) -> u8 {
        *memory.get(address).unwrap()
    }

    pub fn write(memory: &mut Vec<u8>, address: usize, data: u8) {
       *memory.get_mut(address).unwrap() = data;
    }
}
