use core::ops::{Add, Mul, Neg, Not};

use crate::{Vec0, Vector, StaticList};



impl<T> Neg for Vec0<T>
where T: Neg
{
    type Output = Vec0<<T as Neg>::Output>;
    #[inline]
    fn neg(self) -> Self::Output {
        Vec0::VALUE
    }
}

impl<T, Inner: StaticList<T>> Neg for Vector<T, Inner>
where
    Inner: Neg,
    T: Neg,
    <Inner as Neg>::Output: StaticList<<T as Neg>::Output>,
{
    type Output = Vector<<T as Neg>::Output, <Inner as Neg>::Output>;
    #[inline]
    fn neg(self) -> Self::Output {
        Vector(-self.0, -self.1)
    }
}


impl<T> Not for Vec0<T>
where T: Not
{
    type Output = Vec0<<T as Not>::Output>;
    #[inline]
    fn not(self) -> Self::Output {
        Vec0::VALUE
    }
}

impl<T, Inner: StaticList<T>> Not for Vector<T, Inner>
where
    T: Not,
    Inner: Not,
    <Inner as Not>::Output: StaticList<<T as Not>::Output>,
{
    type Output = Vector<<T as Not>::Output, <Inner as Not>::Output>;
    #[inline]
    fn not(self) -> Self::Output {
        Vector(!self.0, !self.1)
    }
}



impl<LeftT, RightT> Add<Vec0<RightT>> for Vec0<LeftT>
where LeftT: Add<RightT>
{
    type Output = Vec0<<LeftT as Add<RightT>>::Output>;
    #[inline]
    fn add(self, _rhs: Vec0<RightT>) -> Self::Output {
        Vec0::VALUE
    }
}

impl<LeftT, LeftInner: StaticList<LeftT>, RightT, RightInner: StaticList<RightT>> Add<Vector<RightT, RightInner>> for Vector<LeftT, LeftInner>
where
    LeftT: Add<RightT>,
    LeftInner: Add<RightInner>,
    <LeftInner as Add<RightInner>>::Output: StaticList<<LeftT as Add<RightT>>::Output>,
{
    type Output = Vector<<LeftT as Add<RightT>>::Output, <LeftInner as Add<RightInner>>::Output>;
    #[inline]
    fn add(self, rhs: Vector<RightT, RightInner>) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}



impl<LeftT, RightT> Mul<RightT> for Vec0<LeftT>
where LeftT: Mul<RightT>
{
    type Output = Vec0<<LeftT as Mul<RightT>>::Output>;
    #[inline]
    fn mul(self, _rhs: RightT) -> Self::Output {
        Vec0::VALUE
    }
}

impl<LeftT, LeftInner: StaticList<LeftT>, RightT> Mul<RightT> for Vector<LeftT, LeftInner>
where
    LeftT: Mul<RightT>,
    RightT: Copy,
    LeftInner: Mul<RightT>,
    <LeftInner as Mul<RightT>>::Output: StaticList<<LeftT as Mul<RightT>>::Output>,
{
    type Output = Vector<<LeftT as Mul<RightT>>::Output, <LeftInner as Mul<RightT>>::Output>;
    #[inline]
    fn mul(self, rhs: RightT) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs)
    }
}

