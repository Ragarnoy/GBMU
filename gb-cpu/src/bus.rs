pub trait Bus<T> {
    type Item;
    type Result;
    type Data;

    fn get(&self, position: T) -> Self::Item;
    fn set(&mut self, position: T, data: Self::Data) -> Self::Result;
}
