pub type Lock = gb_bus::Source;

pub trait Lockable {
    fn lock(&mut self, owner: Lock);
    fn unlock(&mut self);
    fn get_lock(&self) -> Option<Lock>;
}
