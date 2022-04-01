const DUTY_CYCLES: [[bool; 8]; 4] = [
    [false, false, false, false, false, false, false, true],
    [true, false, false, false, false, false, false, true],
    [true, false, false, false, false, true, true, true],
    [false, true, true, true, true, true, true, false],
];

#[derive(Default)]
pub struct Duty {
    pub pattern_index: u8,
    step: u8,
}

impl Duty {
    pub fn reset(&mut self) {
        self.step = 0;
    }
    pub fn next(&mut self) -> bool {
        self.step += 1;
        self.step %= 8;
        DUTY_CYCLES[self.pattern_index as usize][self.step as usize]
    }
}
