use crate::{StaticMinus, Count, CountTrait};


pub trait StaticList<T> {
    type Count: CountTrait;
    const LENGTH: usize = Self::Count::VALUE;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a;
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a;
    fn iter_owned(self) -> impl Iterator<Item = T>;
}


pub trait StaticIndexFromEnd<T, C: CountTrait> {
    fn static_index_from_end(&self) -> &T;
    fn static_index_from_end_mut(&mut self) -> &mut T;
    fn static_index_from_end_owned(self) -> T;
}


pub trait StaticIndex<T, C>:
    StaticList<T, Count = Count<Self::LengthMinusOne>> + 
    StaticIndexFromEnd<T, <Self::LengthMinusOne as StaticMinus<C>>::Difference>
where C: CountTrait
{
    type LengthMinusOne: CountTrait + StaticMinus<C>;
    fn static_index(&self) -> &T {
        <Self as StaticIndexFromEnd<T, <Self::LengthMinusOne as StaticMinus<C>>::Difference>>::static_index_from_end(self)
    }
    fn static_index_mut(&mut self) -> &mut T {
        <Self as StaticIndexFromEnd<T, <Self::LengthMinusOne as StaticMinus<C>>::Difference>>::static_index_from_end_mut(self)
    }
    fn static_index_owned(self) -> T where Self: Sized {
        <Self as StaticIndexFromEnd<T, <Self::LengthMinusOne as StaticMinus<C>>::Difference>>::static_index_from_end_owned(self)
    }
}

