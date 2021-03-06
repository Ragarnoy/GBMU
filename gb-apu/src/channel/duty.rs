const DUTY_CYCLES_SIZE: usize = 8;
const DUTY_CYCLES: [[u8; DUTY_CYCLES_SIZE]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

#[derive(Default, Debug)]
pub struct Duty {
    pub pattern_index: u8,
    step: u8,
}

impl Duty {
    pub fn reset(&mut self) {
        self.step = 0;
    }

    pub fn step(&mut self) {
        self.step = (self.step + 1) % DUTY_CYCLES_SIZE as u8;
    }

    pub fn get_amplitude(&self) -> u8 {
        DUTY_CYCLES[self.pattern_index as usize][self.step as usize]
    }
}
