use crate::{number::{Num0, NumAdd1, Number, StaticMinus}, recursive_list::{RecursiveCase, RecursiveList, RecursiveListMut, RecursiveListOwned}, static_list::{StaticList, static_index::{StaticIndex, StaticIndexMut, StaticIndexOwned}}};


/// Get a `ref` to the `Index`th element, counting from the end of the list. Used internally to implement regular [`StaticIndex`].
pub trait StaticIndexFromEnd<T, Index>
where
    Index: Number,
{
    fn static_index_from_end(&self) -> &T;
}

/// Get a `ref mut` to the `Index`th element, counting from the end of the list. Used internally to implement regular [`StaticIndexMut`].
pub trait StaticIndexFromEndMut<T, Index>:
    StaticIndexFromEnd<T, Index>
where
    Index: Number,
{
    fn static_index_from_end_mut(&mut self) -> &mut T;
}

/// Get the `Index`th element, counting from the end of the list. Used internally to implement regular [`StaticIndexOwned`].
pub trait StaticIndexFromEndOwned<T, Index: Number>:
    StaticIndexFromEnd<T, Index>
where
    Index: Number,
{
    fn static_index_from_end_owned(self) -> T;
}


impl<T, List> StaticIndexFromEnd<T, Num0> for List
where
    List: RecursiveList<T, Case = RecursiveCase>,
{
    #[inline]
    fn static_index_from_end(&self) -> &T {
        self.contents().end
    }
}

impl<T, List> StaticIndexFromEndMut<T, Num0> for List
where
    List: RecursiveListMut<T, Case = RecursiveCase>,
{
    #[inline]
    fn static_index_from_end_mut(&mut self) -> &mut T {
        self.contents_mut().end
    }
}

impl<T, List> StaticIndexFromEndOwned<T, Num0> for List
where
    List: RecursiveListOwned<T, Case = RecursiveCase>,
{
    #[inline]
    fn static_index_from_end_owned(self) -> T {
        self.contents_owned().end
    }
}


impl<T, Index, List> StaticIndexFromEnd<T, NumAdd1<Index>> for List
where
    Index: Number,
    List: RecursiveList<T, Case = RecursiveCase>,
    List::Inner: StaticIndexFromEnd<T, Index>,
{
    #[inline]
    fn static_index_from_end(&self) -> &T {
        StaticIndexFromEnd::<T, Index>::static_index_from_end(self.contents().inner)
    }
}

impl<T, Index, List> StaticIndexFromEndMut<T, NumAdd1<Index>> for List
where
    Index: Number,
    List: RecursiveListMut<T, Case = RecursiveCase>,
    List::Inner: StaticIndexFromEndMut<T, Index>,
{
    #[inline]
    fn static_index_from_end_mut(&mut self) -> &mut T {
        StaticIndexFromEndMut::<T, Index>::static_index_from_end_mut(self.contents_mut().inner)
    }
}

impl<T, Index, List> StaticIndexFromEndOwned<T, NumAdd1<Index>> for List
where
    Index: Number,
    List: RecursiveListOwned<T, Case = RecursiveCase>,
    List::Inner: StaticIndexFromEndOwned<T, Index>,
{
    #[inline]
    fn static_index_from_end_owned(self) -> T {
        StaticIndexFromEndOwned::<T, Index>::static_index_from_end_owned(self.contents_owned().inner)
    }
}


impl<T, Index, List> StaticIndex<T, Index> for List
where
    Index: Number,
    List: RecursiveList<T>,
    <List::Inner as StaticList<T>>::Length: StaticMinus<Index>,
    List::Length: StaticMinus<NumAdd1<Index>>,
    List: StaticIndexFromEnd<T, <List::Length as StaticMinus<NumAdd1<Index>>>::Difference>,
{
    #[inline]
    fn static_index(&self) -> &T {
        <List as StaticIndexFromEnd<T, <List::Length as StaticMinus<NumAdd1<Index>>>::Difference>>::static_index_from_end(self)
    }
}

impl<T, Index, List> StaticIndexMut<T, Index> for List
where
    Index: Number,
    List: RecursiveListMut<T>,
    <List::Inner as StaticList<T>>::Length: StaticMinus<Index>,
    List::Length: StaticMinus<NumAdd1<Index>>,
    List: StaticIndexFromEndMut<T, <List::Length as StaticMinus<NumAdd1<Index>>>::Difference>,
{
    #[inline]
    fn static_index_mut(&mut self) -> &mut T {
        <List as StaticIndexFromEndMut<T, <List::Length as StaticMinus<NumAdd1<Index>>>::Difference>>::static_index_from_end_mut(self)
    }
}

impl<T, Index, List> StaticIndexOwned<T, Index> for List
where
    Index: Number,
    List: RecursiveListOwned<T>,
    <List::Inner as StaticList<T>>::Length: StaticMinus<Index>,
    List::Length: StaticMinus<NumAdd1<Index>>,
    List: StaticIndexFromEndOwned<T, <List::Length as StaticMinus<NumAdd1<Index>>>::Difference>,
{
    #[inline]
    fn static_index_owned(self) -> T {
        <List as StaticIndexFromEndOwned<T, <List::Length as StaticMinus<NumAdd1<Index>>>::Difference>>::static_index_from_end_owned(self)
    }
}

