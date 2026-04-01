use crate::{iterable::Iterable, matrix::Matrix, static_list::StaticList};


impl<
    T,
    Column: StaticList<T>,
    Inner: StaticList<Column>,
>
    Matrix<T, Column, Inner>
{
    fn fmt_from_fn<'a, F>(&'a self, f: &mut core::fmt::Formatter<'_>, fmt: F) -> core::fmt::Result
    where
        T: 'a,
        Column: Iterable<T>,
        Inner: Iterable<Column>,
        F: Fn(&'a T, &mut core::fmt::Formatter<'_>) -> core::fmt::Result,
    {
        use crate::number::Number;
        let rows = <Self as StaticList<Column>>::Length::VALUE;
        let columns = <Column as StaticList<T>>::Length::VALUE;
        
        write!(f, "Mat{rows}")?;
        if rows != columns {
            write!(f, "x{columns}")?;
        }
        writeln!(f, " ( ## Column major! [todo] ##")?;
        
        for column in self.iter() {
            write!(f, "\t[\t")?;
            for (i, value) in column.iter().enumerate() {
                if i > 0 { write!(f, ",\t")?; }
                fmt(value, f)?;
            }
            writeln!(f, "\t],")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<'a, T: core::fmt::Binary  , Column: StaticList<T> + Iterable<T>, Inner: StaticList<Column> + Iterable<Column>> core::fmt::Binary   for Matrix<T, Column, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Binary  >::fmt) }}
impl<'a, T: core::fmt::Debug   , Column: StaticList<T> + Iterable<T>, Inner: StaticList<Column> + Iterable<Column>> core::fmt::Debug    for Matrix<T, Column, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Debug   >::fmt) }}
impl<'a, T: core::fmt::Display , Column: StaticList<T> + Iterable<T>, Inner: StaticList<Column> + Iterable<Column>> core::fmt::Display  for Matrix<T, Column, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Display >::fmt) }}
impl<'a, T: core::fmt::LowerExp, Column: StaticList<T> + Iterable<T>, Inner: StaticList<Column> + Iterable<Column>> core::fmt::LowerExp for Matrix<T, Column, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::LowerExp>::fmt) }}
impl<'a, T: core::fmt::LowerHex, Column: StaticList<T> + Iterable<T>, Inner: StaticList<Column> + Iterable<Column>> core::fmt::LowerHex for Matrix<T, Column, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::LowerHex>::fmt) }}
impl<'a, T: core::fmt::Octal   , Column: StaticList<T> + Iterable<T>, Inner: StaticList<Column> + Iterable<Column>> core::fmt::Octal    for Matrix<T, Column, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Octal   >::fmt) }}
impl<'a, T: core::fmt::Pointer , Column: StaticList<T> + Iterable<T>, Inner: StaticList<Column> + Iterable<Column>> core::fmt::Pointer  for Matrix<T, Column, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Pointer >::fmt) }}
impl<'a, T: core::fmt::UpperExp, Column: StaticList<T> + Iterable<T>, Inner: StaticList<Column> + Iterable<Column>> core::fmt::UpperExp for Matrix<T, Column, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::UpperExp>::fmt) }}
impl<'a, T: core::fmt::UpperHex, Column: StaticList<T> + Iterable<T>, Inner: StaticList<Column> + Iterable<Column>> core::fmt::UpperHex for Matrix<T, Column, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::UpperHex>::fmt) }}

