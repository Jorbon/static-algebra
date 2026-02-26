use std::marker::PhantomData;
use num_traits::{ConstZero, Zero};

use crate::{StaticList, Vec0, Vector};


impl<T, Inner: StaticList<T>> std::fmt::Debug for Vector<T, Inner>
where T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec{}( ", Self::LENGTH)?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{:?}", value)?;
        }
        write!(f, " )")?;
        Ok(())
    }
}

impl<T, Inner: StaticList<T>> std::fmt::Display for Vector<T, Inner>
where T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec{}( ", Self::LENGTH)?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", value)?;
        }
        write!(f, " )")?;
        Ok(())
    }
}

impl<T> Zero for Vec0<T>
where T: Zero
{
    fn zero() -> Self {
        Self(PhantomData)
    }
    fn is_zero(&self) -> bool {
        true
    }
}
impl<T, Inner: StaticList<T>> Zero for Vector<T, Inner>
where
    Inner: Zero,
    T: Zero,
{
    fn zero() -> Self {
        Self(Inner::zero(), T::zero())
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero()
    }
}

impl<T> ConstZero for Vec0<T>
where T: ConstZero
{
    const ZERO: Self = Self(PhantomData);
}
impl<T, Inner: StaticList<T>> ConstZero for Vector<T, Inner>
where
    Inner: ConstZero,
    T: ConstZero,
{
    const ZERO: Self = Vector(Inner::ZERO, T::ZERO);
}


impl<T> From<()> for Vec0<T> {
    fn from(_value: ()) -> Self {
        Self(PhantomData)
    }
}
impl<T, Inner: StaticList<T>, IntoT: Into<T>, IntoInner: Into<Inner>> From<(IntoInner, IntoT)> for Vector<T, Inner> {
    fn from(value: (IntoInner, IntoT)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

