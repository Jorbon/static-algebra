use crate::{number::{NumAdd1, Number, StaticMinus}, static_list::{StaticList, StaticListMut, StaticListOwned}};


pub trait StaticIndex<T, Index>:
    StaticList<T>
where
    Index: Number,
    Self::Length: StaticMinus<NumAdd1<Index>>,
{
    fn static_index(&self) -> &T;
}

pub trait StaticIndexMut<T, Index>:
    StaticListMut<T> +
    StaticIndex<T, Index>
where
    Index: Number,
    Self::Length: StaticMinus<NumAdd1<Index>>,
{
    fn static_index_mut(&mut self) -> &mut T;
}

pub trait StaticIndexOwned<T, Index>:
    StaticListOwned<T> +
    StaticIndex<T, Index>
where
    Index: Number,
    Self::Length: StaticMinus<NumAdd1<Index>>,
{
    fn static_index_owned(self) -> T;
}

