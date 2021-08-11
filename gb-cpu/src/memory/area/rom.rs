pub mod mbc;

pub use mbc::Mbc;
pub use mbc::NoMbc;

pub trait Rom {
    type Item;
    type Result;

    fn get(&self, address: usize) -> Self::Item;
    fn set(&mut self, address: usize, data: Self::Item) -> Self::Result;
}
