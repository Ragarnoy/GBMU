pub trait Get<T> {
    type Output;

    fn get(&self, _: T) -> Self::Output;
}

pub trait Set<T> {
    type Output;
    type Data;

    fn set(&mut self, _: T, data: Self::Data) -> Self::Output;
}
