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

impl<
        T,
        Inner: StaticList<T>,
    >
    Zero
for Vector<T, Inner>
where
    T: Zero,
    Inner: Zero,
    Self: core::ops::Add<Self, Output = Self>,
{
    #[inline]
    fn zero() -> Self {
        Self(Inner::zero(), T::zero())
    }
    
    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero()
    }
}


impl<T> ConstZero for Vec0<T>
where
    T: ConstZero,
    Self: Zero,
{
    const ZERO: Self = Vec0::VALUE;
}

impl<
    T,
    Inner: StaticList<T>,
>
    ConstZero
for Vector<T, Inner>
where
    T: ConstZero,
    Inner: ConstZero,
    Self: Zero,
{
    const ZERO: Self = Vector(Inner::ZERO, T::ZERO);
}

