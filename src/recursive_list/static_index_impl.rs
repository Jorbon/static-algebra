use crate::{number::{Num0, NumAdd1, Number, StaticMinus}, recursive_list::{RecursiveCase, RecursiveList}, static_list::StaticGet};


/// Get the `Index`th element, counting from the end of the list. Used internally to implement regular static indexing.
pub trait StaticIndexFromEnd<T, Index>
where
    Index: Number,
{
    fn static_index_from_end(self) -> T;
}


impl<List> StaticIndexFromEnd<List::T, Num0> for List
where
    List: RecursiveList<Case = RecursiveCase>,
{
    #[inline]
    fn static_index_from_end(self) -> List::T {
        self.contents().end
    }
}

impl<List, Index> StaticIndexFromEnd<List::T, NumAdd1<Index>> for List
where
    Index: Number,
    List: RecursiveList<Case = RecursiveCase>,
    List::Inner: StaticIndexFromEnd<List::T, Index>,
{
    #[inline]
    fn static_index_from_end(self) -> List::T {
        StaticIndexFromEnd::<List::T, Index>::static_index_from_end(self.contents().inner)
    }
}


impl<List, Index> StaticGet<List::T, Index> for List
where
    Index: Number,
    List: RecursiveList,
    List::Length: StaticMinus<NumAdd1<Index>>,
    List: StaticIndexFromEnd<List::T, <List::Length as StaticMinus<NumAdd1<Index>>>::Difference>,
{
    #[inline]
    fn static_get(self) -> List::T {
        <List as StaticIndexFromEnd<List::T, <List::Length as StaticMinus<NumAdd1<Index>>>::Difference>>::static_index_from_end(self)
    }
}

