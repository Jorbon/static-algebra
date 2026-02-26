use crate::{StaticMinus, Add1, Number};


pub trait StaticList<T> {
    type Length: Number;
    const LENGTH: usize = Self::Length::VALUE;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a;
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a;
    fn iter_owned(self) -> impl Iterator<Item = T>;
}


pub trait StaticIndexFromEnd<T, N: Number> {
    fn static_index_from_end(&self) -> &T;
    fn static_index_from_end_mut(&mut self) -> &mut T;
    fn static_index_from_end_owned(self) -> T;
}


pub trait StaticIndex<T, N: Number>:
    StaticList<T, Length = Add1<Self::LengthMinusOne>> + 
    StaticIndexFromEnd<T, <Self::LengthMinusOne as StaticMinus<N>>::Difference>
{
    type LengthMinusOne: Number + StaticMinus<N>;
    fn static_index(&self) -> &T {
        <Self as StaticIndexFromEnd<T, <Self::LengthMinusOne as StaticMinus<N>>::Difference>>::static_index_from_end(self)
    }
    fn static_index_mut(&mut self) -> &mut T {
        <Self as StaticIndexFromEnd<T, <Self::LengthMinusOne as StaticMinus<N>>::Difference>>::static_index_from_end_mut(self)
    }
    fn static_index_owned(self) -> T where Self: Sized {
        <Self as StaticIndexFromEnd<T, <Self::LengthMinusOne as StaticMinus<N>>::Difference>>::static_index_from_end_owned(self)
    }
}

