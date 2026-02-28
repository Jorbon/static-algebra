use crate::{Matrix, Number, StaticIndex, StaticList};


pub struct MatrixColumnView<'a, T, Row: StaticList<T>, Inner: StaticList<Row>, N: Number>(
    &'a Matrix<T, Row, Inner>,
    core::marker::PhantomData<N>,
) where Row: StaticIndex<T, N>;

impl<T, Row: StaticList<T>, Inner: StaticList<Row>> Matrix<T, Row, Inner> {
    
    #[inline]
    pub fn row<'a, N: Number>(&'a self) -> &'a Row
    where Self: StaticIndex<Row, N>
    {
        StaticIndex::<Row, N>::static_index(self)
    }
    
    #[inline]
    pub fn column<'a, N: Number>(&'a self) -> MatrixColumnView<'a, T, Row, Inner, N>
    where Row: StaticIndex<T, N>
    {
        MatrixColumnView(self, core::marker::PhantomData)
    }
}

