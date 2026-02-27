use num_traits::{ConstZero, Zero};

use crate::{StaticList, Vec0, Vector, vec0};


impl<T> Zero for Vec0<T>
where T: Zero
{
    fn zero() -> Self {
        vec0()
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
    const ZERO: Self = vec0();
}
impl<T, Inner: StaticList<T>> ConstZero for Vector<T, Inner>
where
    Inner: ConstZero,
    T: ConstZero,
{
    const ZERO: Self = Vector(Inner::ZERO, T::ZERO);
}

