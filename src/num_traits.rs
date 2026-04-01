use num_traits::{ConstZero, Zero};

use crate::{static_list::StaticList, vector::{Vec0, Vector}};


impl<T> Zero for Vec0<T>
where
    T: Zero,
    Self: core::ops::Add<Self, Output = Self>,
{
    #[inline]
    fn zero() -> Self {
        Vec0::VALUE
    }
    
    #[inline]
    fn is_zero(&self) -> bool {
        true
    }
}

impl<T, Inner> Zero for Vector<T, Inner>
where
    T: Zero,
    Inner: StaticList<T> + Zero,
    Self: core::ops::Add<Self, Output = Self>,
{
    #[inline]
    fn zero() -> Self {
        Self::push(Inner::zero(), T::zero())
    }
    
    #[inline]
    fn is_zero(&self) -> bool {
        self.inner.is_zero() && self.end.is_zero()
    }
}


impl<T> ConstZero for Vec0<T>
where
    T: ConstZero,
    Self: Zero,
{
    const ZERO: Self = Vec0::VALUE;
}

impl<T, Inner> ConstZero for Vector<T, Inner>
where
    T: ConstZero,
    Inner: StaticList<T> + ConstZero,
    Self: Zero,
{
    const ZERO: Self = Vector::push(Inner::ZERO, T::ZERO);
}

