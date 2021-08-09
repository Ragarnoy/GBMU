const WRAM_SIZE: usize = 8192;

#[derive(Debug)]
pub struct Wram {
    data: [u8; WRAM_SIZE],
}

impl Default for Wram {
    fn default() -> Self {
        Self::new()
    }
}

impl Wram {
    pub fn new() -> Self {
        Wram {
            data: [0; WRAM_SIZE],
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        *self.data.get(address).unwrap()
    }

    pub fn write(&mut self, address: usize, data: u8) {
        self.data[address] = data;
    }
}

#[cfg(test)]
mod test_wram {
    use super::Wram;

    #[test]
    fn test_read_wram() {
        let wram = Wram::default();

        assert_eq!(wram.read(0x10), 0);
    }

    #[test]
    fn test_write_read_wram() {
        let mut wram = Wram::default();

        wram.write(0x42, 42);
        let read = wram.read(0x42);

        assert_eq!(read, 42);
    }
}
