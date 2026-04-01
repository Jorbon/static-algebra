use crate::{matrix::{Mat0, Matrix}, static_list::StaticList};


impl<T, Column> From<()> for Mat0<T, Column>
where
    Column: StaticList<T>,
{
    #[inline]
    fn from(_value: ()) -> Self {
        Mat0::VALUE
    }
}

impl<T, Column, Inner, IntoColumn, IntoInner> From<(IntoInner, IntoColumn)> for Matrix<T, Column, Inner>
where
    Column: StaticList<T>,
    Inner: StaticList<Column>,
    IntoColumn: Into<Column>,
    IntoInner: Into<Inner>,
{
    #[inline]
    fn from(value: (IntoInner, IntoColumn)) -> Self {
        Self::push_column(value.0.into(), value.1.into())
    }
}

// impl<> From<[IntoColumn; 1]> for Matrix<T, Column, Inner> {
    
// }