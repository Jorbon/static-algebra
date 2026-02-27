use crate::{Iterable, Number, StaticList, Vec0, Vector, vec0};


impl<T> From<()> for Vec0<T> {
    #[inline]
    fn from(_value: ()) -> Self {
        vec0()
    }
}

impl<T, Inner: StaticList<T>, IntoT: Into<T>, IntoInner: Into<Inner>> From<(IntoInner, IntoT)> for Vector<T, Inner> {
    #[inline]
    fn from(value: (IntoInner, IntoT)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}


impl<'a, T, Inner: StaticList<T>> core::fmt::Debug for Vector<T, Inner>
where
    T: core::fmt::Debug + 'a,
    Inner: Iterable<T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vec{}( ", <<Self as StaticList<T>>::Length as Number>::VALUE)?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{:?}", value)?;
        }
        write!(f, " )")?;
        Ok(())
    }
}

impl<'a, T, Inner: StaticList<T>> core::fmt::Display for Vector<T, Inner>
where
    T: core::fmt::Display + 'a,
    Inner: Iterable<T>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vec{}( ", <<Self as StaticList<T>>::Length as Number>::VALUE)?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", value)?;
        }
        write!(f, " )")?;
        Ok(())
    }
}

