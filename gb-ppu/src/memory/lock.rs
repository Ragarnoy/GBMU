#[derive(Debug, Clone, Copy)]
pub enum Lock {
    Ppu,
}

pub trait Lockable {
    fn lock(&mut self, owner: Lock);
    fn unlock(&mut self);
    fn get_lock(&self) -> Option<Lock>;
}
