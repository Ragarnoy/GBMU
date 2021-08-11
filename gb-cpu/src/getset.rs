pub trait Get<T> {
    type Item;

    fn get(&self, _: T) -> Self::Item;
}

pub trait Set<T>: Get<T> {
    type Result;
    type Data;

    fn set(&mut self, _: T, data: Self::Data) -> Self::Result;
}
