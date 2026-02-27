use crate::{StaticList, Vec0, Vector, vec0};


impl<T, Inner: StaticList<T>> core::fmt::Debug for Vector<T, Inner>
where T: core::fmt::Debug
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vec{}( ", Self::LENGTH)?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{:?}", value)?;
        }
        write!(f, " )")?;
        Ok(())
    }
}

impl<T, Inner: StaticList<T>> core::fmt::Display for Vector<T, Inner>
where T: core::fmt::Display
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vec{}( ", Self::LENGTH)?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", value)?;
        }
        write!(f, " )")?;
        Ok(())
    }
}


impl<T> From<()> for Vec0<T> {
    fn from(_value: ()) -> Self {
        vec0()
    }
}
impl<T, Inner: StaticList<T>, IntoT: Into<T>, IntoInner: Into<Inner>> From<(IntoInner, IntoT)> for Vector<T, Inner> {
    fn from(value: (IntoInner, IntoT)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

