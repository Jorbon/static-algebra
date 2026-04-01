//! Allow statically indexing into all [`StaticList`] implementations using a [`Number`] index.

use crate::{number::{Add1, Num0, Number, StaticMinus}, static_list::{StaticList, StaticListRecursive, StaticListRecursiveMut, StaticListRecursiveOwned}};


/// Get a `ref` to the `Index`th element, counting from the end of the list. Used internally to implement regular [`StaticIndex`].
pub trait StaticIndexFromEnd<T, Index: Number> {
    fn static_index_from_end(&self) -> &T;
}

/// Get a `ref mut` to the `Index`th element, counting from the end of the list. Used internally to implement regular [`StaticIndexMut`].
pub trait StaticIndexFromEndMut<T, Index: Number> {
    fn static_index_from_end_mut(&mut self) -> &mut T;
}

/// Get the `Index`th element, counting from the end of the list. Used internally to implement regular [`StaticIndexOwned`].
pub trait StaticIndexFromEndOwned<T, Index: Number> {
    fn static_index_from_end_owned(self) -> T;
}


impl<T, List> StaticIndexFromEnd<T, Num0> for List
where
    List: StaticListRecursive<T>,
{
    #[inline]
    fn static_index_from_end(&self) -> &T {
        self.parts().end
    }
}

impl<T, List> StaticIndexFromEndMut<T, Num0> for List
where
    List: StaticListRecursiveMut<T>,
{
    #[inline]
    fn static_index_from_end_mut(&mut self) -> &mut T {
        self.parts_mut().end
    }
}

impl<T, List> StaticIndexFromEndOwned<T, Num0> for List
where
    List: StaticListRecursiveOwned<T>,
{
    #[inline]
    fn static_index_from_end_owned(self) -> T {
        self.parts_owned().end
    }
}



impl<T, Index, List> StaticIndexFromEnd<T, Add1<Index>> for List
where
    Index: Number,
    List: StaticListRecursive<T>,
    <List as StaticListRecursive<T>>::Inner: StaticIndexFromEnd<T, Index>,
{
    #[inline]
    fn static_index_from_end(&self) -> &T {
        StaticIndexFromEnd::<T, Index>::static_index_from_end(self.parts().inner)
    }
}

impl<T, Index, List> StaticIndexFromEndMut<T, Add1<Index>> for List
where
    Index: Number,
    List: StaticListRecursiveMut<T>,
    <List as StaticListRecursive<T>>::Inner: StaticIndexFromEndMut<T, Index>,
{
    #[inline]
    fn static_index_from_end_mut(&mut self) -> &mut T {
        StaticIndexFromEndMut::<T, Index>::static_index_from_end_mut(self.parts_mut().inner)
    }
}

impl<T, Index, List> StaticIndexFromEndOwned<T, Add1<Index>> for List
where
    Index: Number,
    List: StaticListRecursiveOwned<T>,
    <List as StaticListRecursiveOwned<T>>::Inner: StaticIndexFromEndOwned<T, Index>,
{
    #[inline]
    fn static_index_from_end_owned(self) -> T {
        StaticIndexFromEndOwned::<T, Index>::static_index_from_end_owned(self.parts_owned().inner)
    }
}



pub trait StaticIndex<T, Index: Number> {
    fn static_index(&self) -> &T;
}

pub trait StaticIndexMut<T, Index: Number> {
    fn static_index_mut(&mut self) -> &mut T;
}

pub trait StaticIndexOwned<T, Index: Number> {
    fn static_index_owned(self) -> T;
}



impl<T, Index, List> StaticIndex<T, Index> for List
where
    Index: Number,
    List: StaticListRecursive<T> + StaticIndexFromEnd<T, <<<List as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>,
    <<List as StaticListRecursive<T>>::Inner as StaticList<T>>::Length: StaticMinus<Index>,
{
    #[inline]
    fn static_index(&self) -> &T {
        <List as StaticIndexFromEnd<T, <<<List as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>>::static_index_from_end(self)
    }
}

impl<T, Index, List> StaticIndexMut<T, Index> for List
where
    Index: Number,
    List: StaticListRecursive<T> + StaticIndexFromEndMut<T, <<<List as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>,
    <<List as StaticListRecursive<T>>::Inner as StaticList<T>>::Length: StaticMinus<Index>,
{
    #[inline]
    fn static_index_mut(&mut self) -> &mut T {
        <List as StaticIndexFromEndMut<T, <<<List as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>>::static_index_from_end_mut(self)
    }
}

impl<T, Index, List> StaticIndexOwned<T, Index> for List
where
    Index: Number,
    List: StaticListRecursive<T> + StaticIndexFromEndOwned<T, <<<List as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>,
    <<List as StaticListRecursive<T>>::Inner as StaticList<T>>::Length: StaticMinus<Index>,
{
    #[inline]
    fn static_index_owned(self) -> T {
        <List as StaticIndexFromEndOwned<T, <<<List as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>>::static_index_from_end_owned(self)
    }
}

