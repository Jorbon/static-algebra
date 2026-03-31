use crate::{Iterable, Number, StaticList, Vector};


impl<
    T,
    Inner: StaticList<T>,
>
    Vector<T, Inner>
{
    fn fmt_from_fn<'a, F>(&'a self, f: &mut core::fmt::Formatter<'_>, fmt: F) -> core::fmt::Result
    where
        T: 'a,
        Inner: Iterable<T>,
        F: Fn(&'a T, &mut core::fmt::Formatter<'_>) -> core::fmt::Result,
    {
        write!(f, "Vec{}( ", <<Self as StaticList<T>>::Length as Number>::VALUE)?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            fmt(value, f)?;
        }
        write!(f, " )")?;
        Ok(())
    }
}

impl<'a, T: core::fmt::Binary  , Inner: StaticList<T> + Iterable<T>> core::fmt::Binary   for Vector<T, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Binary  >::fmt) }}
impl<'a, T: core::fmt::Debug   , Inner: StaticList<T> + Iterable<T>> core::fmt::Debug    for Vector<T, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Debug   >::fmt) }}
impl<'a, T: core::fmt::Display , Inner: StaticList<T> + Iterable<T>> core::fmt::Display  for Vector<T, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Display >::fmt) }}
impl<'a, T: core::fmt::LowerExp, Inner: StaticList<T> + Iterable<T>> core::fmt::LowerExp for Vector<T, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::LowerExp>::fmt) }}
impl<'a, T: core::fmt::LowerHex, Inner: StaticList<T> + Iterable<T>> core::fmt::LowerHex for Vector<T, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::LowerHex>::fmt) }}
impl<'a, T: core::fmt::Octal   , Inner: StaticList<T> + Iterable<T>> core::fmt::Octal    for Vector<T, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Octal   >::fmt) }}
impl<'a, T: core::fmt::Pointer , Inner: StaticList<T> + Iterable<T>> core::fmt::Pointer  for Vector<T, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Pointer >::fmt) }}
impl<'a, T: core::fmt::UpperExp, Inner: StaticList<T> + Iterable<T>> core::fmt::UpperExp for Vector<T, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::UpperExp>::fmt) }}
impl<'a, T: core::fmt::UpperHex, Inner: StaticList<T> + Iterable<T>> core::fmt::UpperHex for Vector<T, Inner> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::UpperHex>::fmt) }}

