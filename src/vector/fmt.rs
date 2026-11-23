use crate::{number::Number, static_list::{Iterable, StaticList}, vector::{VecPush, Vector}};


impl<T, Inner> VecPush<Inner>
where
    Inner: Vector<T = T>,
{
    fn fmt_from_fn<'a, F>(&'a self, f: &mut core::fmt::Formatter<'_>, fmt: F) -> core::fmt::Result
    where
        T: 'a,
        F: Fn(&'a T, &mut core::fmt::Formatter<'_>) -> core::fmt::Result,
    {
        write!(f, "Vec{}( ", <<Self as StaticList>::Length as Number>::VALUE)?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            fmt(&value, f)?; // TODO: implement Vector<&T> for &Vector<T>
        }
        write!(f, " )")?;
        Ok(())
    }
}

impl<'a, T, Inner> core::fmt::Binary   for VecPush<Inner> where T: core::fmt::Binary  , Inner: Vector<T = T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Binary  >::fmt) }}
impl<'a, T, Inner> core::fmt::Debug    for VecPush<Inner> where T: core::fmt::Debug   , Inner: Vector<T = T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Debug   >::fmt) }}
impl<'a, T, Inner> core::fmt::Display  for VecPush<Inner> where T: core::fmt::Display , Inner: Vector<T = T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Display >::fmt) }}
impl<'a, T, Inner> core::fmt::LowerExp for VecPush<Inner> where T: core::fmt::LowerExp, Inner: Vector<T = T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::LowerExp>::fmt) }}
impl<'a, T, Inner> core::fmt::LowerHex for VecPush<Inner> where T: core::fmt::LowerHex, Inner: Vector<T = T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::LowerHex>::fmt) }}
impl<'a, T, Inner> core::fmt::Octal    for VecPush<Inner> where T: core::fmt::Octal   , Inner: Vector<T = T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Octal   >::fmt) }}
impl<'a, T, Inner> core::fmt::Pointer  for VecPush<Inner> where T: core::fmt::Pointer , Inner: Vector<T = T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::Pointer >::fmt) }}
impl<'a, T, Inner> core::fmt::UpperExp for VecPush<Inner> where T: core::fmt::UpperExp, Inner: Vector<T = T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::UpperExp>::fmt) }}
impl<'a, T, Inner> core::fmt::UpperHex for VecPush<Inner> where T: core::fmt::UpperHex, Inner: Vector<T = T> { fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.fmt_from_fn(f, <T as core::fmt::UpperHex>::fmt) }}

