pub trait Bus<T> {
    type Item;
    type Result;
    type Data;

    fn get(&self, _: T) -> Self::Item;
    fn set(&mut self, _: T, data: Self::Data) -> Self::Result;
}
