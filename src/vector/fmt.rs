use crate::{iterable::Iterable, number::Number, static_list::StaticList, vector::Vector};


impl<T, Inner> Vector<T, Inner>
where
    Inner: StaticList<T> + Iterable<T>,
{
    fn fmt_from_fn<'a, F>(&'a self, f: &mut core::fmt::Formatter<'_>, fmt: F) -> core::fmt::Result
    where
        T: 'a,
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

impl<'a, T, Inner> core::fmt::Binary   for Vector<T, Inner> where T: core::fmt::Binary  , Inner: StaticList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Binary  >::fmt) }}
impl<'a, T, Inner> core::fmt::Debug    for Vector<T, Inner> where T: core::fmt::Debug   , Inner: StaticList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Debug   >::fmt) }}
impl<'a, T, Inner> core::fmt::Display  for Vector<T, Inner> where T: core::fmt::Display , Inner: StaticList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Display >::fmt) }}
impl<'a, T, Inner> core::fmt::LowerExp for Vector<T, Inner> where T: core::fmt::LowerExp, Inner: StaticList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::LowerExp>::fmt) }}
impl<'a, T, Inner> core::fmt::LowerHex for Vector<T, Inner> where T: core::fmt::LowerHex, Inner: StaticList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::LowerHex>::fmt) }}
impl<'a, T, Inner> core::fmt::Octal    for Vector<T, Inner> where T: core::fmt::Octal   , Inner: StaticList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Octal   >::fmt) }}
impl<'a, T, Inner> core::fmt::Pointer  for Vector<T, Inner> where T: core::fmt::Pointer , Inner: StaticList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Pointer >::fmt) }}
impl<'a, T, Inner> core::fmt::UpperExp for Vector<T, Inner> where T: core::fmt::UpperExp, Inner: StaticList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::UpperExp>::fmt) }}
impl<'a, T, Inner> core::fmt::UpperHex for Vector<T, Inner> where T: core::fmt::UpperHex, Inner: StaticList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::UpperHex>::fmt) }}

