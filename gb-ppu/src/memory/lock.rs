#[derive(Debug, Clone, Copy)]
pub enum Lock {
    Ppu,
    #[allow(dead_code)]
    Dma,
}

pub trait Lockable {
    fn lock(&mut self, owner: Option<Lock>);
    fn get_lock(&self) -> Option<Lock>;
}
