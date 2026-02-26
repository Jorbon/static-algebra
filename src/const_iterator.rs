use crate::{ConstMinus, Count, CountTrait};


pub trait ConstIterator<T> {
    type Count: CountTrait;
    const LENGTH: usize = Self::Count::VALUE;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a;
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a;
    fn iter_owned(self) -> impl Iterator<Item = T>;
}

pub trait ConstIndexFromEnd<T, C: CountTrait> {
    fn index_from_end(&self) -> &T;
    fn index_from_end_mut(&mut self) -> &mut T;
    fn index_from_end_owned(self) -> T;
}

pub trait ConstIndex<T, C>:
    ConstIterator<T, Count = Count<Self::LengthMinusOne>> + 
    ConstIndexFromEnd<T, <Self::LengthMinusOne as ConstMinus<C>>::Difference>
where C: CountTrait
{
    type LengthMinusOne: CountTrait + ConstMinus<C>;
    fn index(&self) -> &T {
        <Self as ConstIndexFromEnd<T, <Self::LengthMinusOne as ConstMinus<C>>::Difference>>::index_from_end(self)
    }
    fn index_mut(&mut self) -> &mut T {
        <Self as ConstIndexFromEnd<T, <Self::LengthMinusOne as ConstMinus<C>>::Difference>>::index_from_end_mut(self)
    }
    fn index_owned(self) -> T where Self: Sized {
        <Self as ConstIndexFromEnd<T, <Self::LengthMinusOne as ConstMinus<C>>::Difference>>::index_from_end_owned(self)
    }
}

