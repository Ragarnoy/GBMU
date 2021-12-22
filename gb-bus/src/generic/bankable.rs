use std::ops::{Index, IndexMut};

pub struct BankableStorage<const BANK_COUNT: usize, const BANK_SIZE: usize> {
    banks: [[u8; BANK_SIZE]; BANK_COUNT],
    pub current_bank_index: usize,
}

impl<const BANK_COUNT: usize, const BANK_SIZE: usize> Default
    for BankableStorage<BANK_COUNT, BANK_SIZE>
{
    fn default() -> Self {
        Self {
            banks: [[0; BANK_SIZE]; BANK_COUNT],
            current_bank_index: 0,
        }
    }
}

impl<const BANK_COUNT: usize, const BANK_SIZE: usize> BankableStorage<BANK_COUNT, BANK_SIZE> {
    pub fn set_bank_index(&mut self, index: usize) {
        self.current_bank_index = index;
    }

    pub fn root_bank(&self) -> &[u8; BANK_SIZE] {
        &self.banks[0]
    }

    pub fn root_bank_mut(&mut self) -> &mut [u8; BANK_SIZE] {
        &mut self.banks[0]
    }
}

impl<const BANK_COUNT: usize, const BANK_SIZE: usize> Index<usize>
    for BankableStorage<BANK_COUNT, BANK_SIZE>
{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.banks[self.current_bank_index][index]
    }
}

impl<const BANK_COUNT: usize, const BANK_SIZE: usize> IndexMut<usize>
    for BankableStorage<BANK_COUNT, BANK_SIZE>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.banks[self.current_bank_index][index]
    }
}

impl<const BANK_COUNT: usize, const BANK_SIZE: usize> Index<(usize, usize)>
    for BankableStorage<BANK_COUNT, BANK_SIZE>
{
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (bank_idx, index) = index;
        &self.banks[bank_idx][index]
    }
}

impl<const BANK_COUNT: usize, const BANK_SIZE: usize> IndexMut<(usize, usize)>
    for BankableStorage<BANK_COUNT, BANK_SIZE>
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (bank_idx, index) = index;
        &mut self.banks[bank_idx][index]
    }
}
