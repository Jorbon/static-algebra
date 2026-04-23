use crate::{number::Number, recursive_list::RecursiveList, static_list::{Iterable, StaticList}, vector::{VecPush, Vector}};


impl<T, Inner> VecPush<T, Inner>
where
    Inner: Vector<T> + Iterable<T>,
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

impl<'a, T, Inner> core::fmt::Binary   for VecPush<T, Inner> where T: core::fmt::Binary  , Inner: RecursiveList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Binary  >::fmt) }}
impl<'a, T, Inner> core::fmt::Debug    for VecPush<T, Inner> where T: core::fmt::Debug   , Inner: RecursiveList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Debug   >::fmt) }}
impl<'a, T, Inner> core::fmt::Display  for VecPush<T, Inner> where T: core::fmt::Display , Inner: RecursiveList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Display >::fmt) }}
impl<'a, T, Inner> core::fmt::LowerExp for VecPush<T, Inner> where T: core::fmt::LowerExp, Inner: RecursiveList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::LowerExp>::fmt) }}
impl<'a, T, Inner> core::fmt::LowerHex for VecPush<T, Inner> where T: core::fmt::LowerHex, Inner: RecursiveList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::LowerHex>::fmt) }}
impl<'a, T, Inner> core::fmt::Octal    for VecPush<T, Inner> where T: core::fmt::Octal   , Inner: RecursiveList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Octal   >::fmt) }}
impl<'a, T, Inner> core::fmt::Pointer  for VecPush<T, Inner> where T: core::fmt::Pointer , Inner: RecursiveList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Pointer >::fmt) }}
impl<'a, T, Inner> core::fmt::UpperExp for VecPush<T, Inner> where T: core::fmt::UpperExp, Inner: RecursiveList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::UpperExp>::fmt) }}
impl<'a, T, Inner> core::fmt::UpperHex for VecPush<T, Inner> where T: core::fmt::UpperHex, Inner: RecursiveList<T> + Iterable<T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::UpperHex>::fmt) }}

