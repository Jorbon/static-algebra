use crate::{Add1, Number, Num0, StaticList, StaticListRecursive, StaticListRecursiveMut, StaticListRecursiveOwned, StaticMinus};



pub trait StaticIndexFromEnd<T, Index: Number> {
    fn static_index_from_end(&self) -> &T;
}

pub trait StaticIndexFromEndMut<T, Index: Number> {
    fn static_index_from_end_mut(&mut self) -> &mut T;
}

pub trait StaticIndexFromEndOwned<T, Index: Number> {
    fn static_index_from_end_owned(self) -> T;
}



impl<T, List> StaticIndexFromEnd<T, Num0> for List
where Self: StaticListRecursive<T>
{
    #[inline]
    fn static_index_from_end(&self) -> &T {
        self.end()
    }
}

impl<T, List> StaticIndexFromEndMut<T, Num0> for List
where Self: StaticListRecursiveMut<T>
{
    #[inline]
    fn static_index_from_end_mut(&mut self) -> &mut T {
        self.end_mut()
    }
}

impl<T, List> StaticIndexFromEndOwned<T, Num0> for List
where Self: StaticListRecursiveOwned<T>
{
    #[inline]
    fn static_index_from_end_owned(self) -> T {
        self.end_owned()
    }
}



impl<T, Index: Number, List> StaticIndexFromEnd<T, Add1<Index>> for List
where
    Self: StaticListRecursive<T>,
    <Self as StaticListRecursive<T>>::Inner: StaticIndexFromEnd<T, Index>,
{
    #[inline]
    fn static_index_from_end(&self) -> &T {
        StaticIndexFromEnd::<T, Index>::static_index_from_end(self.inner())
    }
}

impl<T, Index: Number, List> StaticIndexFromEndMut<T, Add1<Index>> for List
where
    Self: StaticListRecursiveMut<T>,
    <Self as StaticListRecursive<T>>::Inner: StaticIndexFromEndMut<T, Index>,
{
    #[inline]
    fn static_index_from_end_mut(&mut self) -> &mut T {
        StaticIndexFromEndMut::<T, Index>::static_index_from_end_mut(self.inner_mut())
    }
}

impl<T, Index: Number, List> StaticIndexFromEndOwned<T, Add1<Index>> for List
where
    Self: StaticListRecursiveOwned<T>,
    <Self as StaticListRecursiveOwned<T>>::Inner: StaticIndexFromEndOwned<T, Index>,
{
    #[inline]
    fn static_index_from_end_owned(self) -> T {
        StaticIndexFromEndOwned::<T, Index>::static_index_from_end_owned(self.inner_owned())
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



impl<T, Index: Number, List> StaticIndex<T, Index> for List
where
    Self: StaticListRecursive<T> + StaticIndexFromEnd<T, <<<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>,
    <<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length: StaticMinus<Index>,
{
    #[inline]
    fn static_index(&self) -> &T {
        <Self as StaticIndexFromEnd<T, <<<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>>::static_index_from_end(self)
    }
}

impl<T, Index: Number, List> StaticIndexMut<T, Index> for List
where
    Self: StaticListRecursive<T> + StaticIndexFromEndMut<T, <<<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>,
    <<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length: StaticMinus<Index>,
{
    #[inline]
    fn static_index_mut(&mut self) -> &mut T {
        <Self as StaticIndexFromEndMut<T, <<<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>>::static_index_from_end_mut(self)
    }
}

impl<T, Index: Number, List> StaticIndexOwned<T, Index> for List
where
    Self: StaticListRecursive<T> + StaticIndexFromEndOwned<T, <<<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>,
    <<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length: StaticMinus<Index>,
{
    #[inline]
    fn static_index_owned(self) -> T {
        <Self as StaticIndexFromEndOwned<T, <<<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length as StaticMinus<Index>>::Difference>>::static_index_from_end_owned(self)
    }
}

