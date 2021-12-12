use std::ops::{Index, IndexMut};

pub struct DynBankableStorage<const BANK_SIZE: usize> {
    banks: Vec<[u8; BANK_SIZE]>,
    pub current_bank_index: usize,
}

impl<const BANK_SIZE: usize> Default for DynBankableStorage<BANK_SIZE> {
    fn default() -> Self {
        Self {
            banks: vec![[0; BANK_SIZE]; 1],
            current_bank_index: 0,
        }
    }
}

impl<const BANK_SIZE: usize> DynBankableStorage<BANK_SIZE> {
    pub fn with_bank_amount(amount: usize) -> Self {
        Self {
            banks: vec![[0; BANK_SIZE]; amount],
            current_bank_index: 0,
        }
    }

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

impl<const BANK_SIZE: usize> Index<usize> for DynBankableStorage<BANK_SIZE> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.banks[self.current_bank_index][index]
    }
}

impl<const BANK_SIZE: usize> IndexMut<usize> for DynBankableStorage<BANK_SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.banks[self.current_bank_index][index]
    }
}

impl<const BANK_SIZE: usize> Index<(usize, usize)> for DynBankableStorage<BANK_SIZE> {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (bank_idx, index) = index;
        &self.banks[bank_idx][index]
    }
}

impl<const BANK_SIZE: usize> IndexMut<(usize, usize)> for DynBankableStorage<BANK_SIZE> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (bank_idx, index) = index;
        &mut self.banks[bank_idx][index]
    }
}
