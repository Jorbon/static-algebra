use std::marker::PhantomData;
use num_traits::{ConstZero, Zero};

use crate::{ConstIterator, Vec0, Vector, VectorTrait};


impl<T, Inner> std::fmt::Debug for Vector<T, Inner>
where
    Inner: VectorTrait<T>,
    T: std::fmt::Debug,
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

impl<T, Inner> std::fmt::Display for Vector<T, Inner>
where
    Inner: VectorTrait<T>,
    T: std::fmt::Display,
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
impl<T, Inner> Zero for Vector<T, Inner>
where
    Inner: VectorTrait<T> + Zero,
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
impl<T, Inner> ConstZero for Vector<T, Inner>
where
    Inner: VectorTrait<T> + ConstZero,
    T: ConstZero,
{
    const ZERO: Self = Vector(Inner::ZERO, T::ZERO);
}


impl<T> From<()> for Vec0<T> {
    #[allow(unused_variables)]
    fn from(value: ()) -> Self {
        Self(PhantomData)
    }
}
impl<T, Inner, IntoT, IntoInner> From<(IntoInner, IntoT)> for Vector<T, Inner>
where
    Inner: VectorTrait<T>,
    IntoT: Into<T>,
    IntoInner: Into<Inner>,
{
    fn from(value: (IntoInner, IntoT)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

