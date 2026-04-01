use crate::{matrix::{Mat0, Matrix}, static_list::StaticList};


impl<
    T,
    Column: StaticList<T>,
>
    From<()>
for Mat0<T, Column>
{
    #[inline]
    fn from(_value: ()) -> Self {
        Mat0::VALUE
    }
}

impl <
    T,
    Column: StaticList<T>,
    Inner: StaticList<Column>,
    IntoColumn: Into<Column>,
    IntoInner: Into<Inner>,
>
    From<(IntoInner, IntoColumn)>
for Matrix<T, Column, Inner>
{
    #[inline]
    fn from(value: (IntoInner, IntoColumn)) -> Self {
        Self::with_inner(value.0.into(), value.1.into())
    }
}

// impl<> From<[IntoColumn; 1]> for Matrix<T, Column, Inner> {
    
// }