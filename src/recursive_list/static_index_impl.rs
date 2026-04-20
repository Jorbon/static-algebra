use crate::{number::{Num0, NumAdd1, Number, StaticMinus}, recursive_list::{RecursiveCase, RecursiveList}, static_list::StaticGet};


/// Get the `Index`th element, counting from the end of the list. Used internally to implement regular static indexing.
pub trait StaticIndexFromEnd<T, Index>
where
    Index: Number,
{
    fn static_index_from_end(self) -> T;
}


impl<T, List> StaticIndexFromEnd<T, Num0> for List
where
    List: RecursiveList<T, Case = RecursiveCase>,
{
    #[inline]
    fn static_index_from_end(self) -> T {
        self.contents().end
    }
}

impl<T, Index, List> StaticIndexFromEnd<T, NumAdd1<Index>> for List
where
    Index: Number,
    List: RecursiveList<T, Case = RecursiveCase>,
    List::Inner: StaticIndexFromEnd<T, Index>,
{
    #[inline]
    fn static_index_from_end(self) -> T {
        StaticIndexFromEnd::<T, Index>::static_index_from_end(self.contents().inner)
    }
}


impl<T, Index, List> StaticGet<T, Index> for List
where
    Index: Number,
    List: RecursiveList<T>,
    List::Length: StaticMinus<NumAdd1<Index>>,
    List: StaticIndexFromEnd<T, <List::Length as StaticMinus<NumAdd1<Index>>>::Difference>,
{
    #[inline]
    fn static_get(self) -> T {
        <List as StaticIndexFromEnd<T, <List::Length as StaticMinus<NumAdd1<Index>>>::Difference>>::static_index_from_end(self)
    }
}

