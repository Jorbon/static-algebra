use core::ops::*;

use crate::{recursive_list::RecursiveList, static_list::{Mappable, StaticMap, StaticZip}, vector::{VecPush, Vector}};


// Element-wise

impl<Inner, T> Neg for VecPush<Inner> where Inner: Vector<T = T>, T: Neg, Self: StaticMap<T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn neg(self) -> Self::Output { self.map(&Neg::neg) } }
impl<Inner, T> Not for VecPush<Inner> where Inner: Vector<T = T>, T: Not, Self: StaticMap<T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn not(self) -> Self::Output { self.map(&Not::not) } }


impl<Inner, Rhs, T> Add   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: Add   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn add   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Add   ::add   ) } }
impl<Inner, Rhs, T> Sub   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: Sub   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn sub   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Sub   ::sub   ) } }
impl<Inner, Rhs, T> BitAnd<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: BitAnd<Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn bitand(self, rhs: Rhs) -> Self::Output { self.zip(rhs, &BitAnd::bitand) } }
impl<Inner, Rhs, T> BitOr <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: BitOr <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn bitor (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &BitOr ::bitor ) } }
impl<Inner, Rhs, T> BitXor<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: BitXor<Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn bitxor(self, rhs: Rhs) -> Self::Output { self.zip(rhs, &BitXor::bitxor) } }
impl<Inner, Rhs, T> Shl   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: Shl   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn shl   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Shl   ::shl   ) } }
impl<Inner, Rhs, T> Shr   <Rhs> for VecPush<Inner> where Inner: Vector<T = T>, Rhs: RecursiveList, T: Shr   <Rhs::T>, Self: StaticZip<Rhs, T, Rhs::T, T::Output> { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn shr   (self, rhs: Rhs) -> Self::Output { self.zip(rhs, &Shr   ::shr   ) } }


// Scalar

impl<Inner, Rhs, T> Mul<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, T: Mul<Rhs>, Self: StaticMap<T, T::Output>, Rhs: Copy { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn mul(self, rhs: Rhs) -> Self::Output { self.map(&|t| T::mul(t, rhs)) } }
impl<Inner, Rhs, T> Div<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, T: Div<Rhs>, Self: StaticMap<T, T::Output>, Rhs: Copy { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn div(self, rhs: Rhs) -> Self::Output { self.map(&|t| T::div(t, rhs)) } }
impl<Inner, Rhs, T> Rem<Rhs> for VecPush<Inner> where Inner: Vector<T = T>, T: Rem<Rhs>, Self: StaticMap<T, T::Output>, Rhs: Copy { type Output = <Self as Mappable<T::Output>>::Mapped; #[inline] fn rem(self, rhs: Rhs) -> Self::Output { self.map(&|t| T::rem(t, rhs)) } }

// Assign

impl<Inner, Rhs, T> AddAssign<Rhs> for VecPush<Inner>
where
    Inner: Vector<T = T>,
    Rhs: RecursiveList,
    T: AddAssign<Rhs::T>,
    for <'a> &'a mut Self: StaticZip<Rhs, &'a mut T, Rhs::T, ()> + Mappable<()>
{
    fn add_assign(&mut self, rhs: Rhs) {
        self.zip(rhs, &AddAssign::add_assign);
    }
}

// impl<T, Inner, U> AddAssign   <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: Clone + Add   <U, Output = Self> { fn add_assign   (&mut self, rhs: U) { *self = self.clone().add   (rhs) }}
// impl<T, Inner, U> SubAssign   <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: Clone + Sub   <U, Output = Self> { fn sub_assign   (&mut self, rhs: U) { *self = self.clone().sub   (rhs) }}
// impl<T, Inner, U> MulAssign   <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: Clone + Mul   <U, Output = Self> { fn mul_assign   (&mut self, rhs: U) { *self = self.clone().mul   (rhs) }}
// impl<T, Inner, U> DivAssign   <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: Clone + Div   <U, Output = Self> { fn div_assign   (&mut self, rhs: U) { *self = self.clone().div   (rhs) }}
// impl<T, Inner, U> RemAssign   <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: Clone + Rem   <U, Output = Self> { fn rem_assign   (&mut self, rhs: U) { *self = self.clone().rem   (rhs) }}
// impl<T, Inner, U> BitAndAssign<U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: Clone + BitAnd<U, Output = Self> { fn bitand_assign(&mut self, rhs: U) { *self = self.clone().bitand(rhs) }}
// impl<T, Inner, U> BitOrAssign <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: Clone + BitOr <U, Output = Self> { fn bitor_assign (&mut self, rhs: U) { *self = self.clone().bitor (rhs) }}
// impl<T, Inner, U> BitXorAssign<U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: Clone + BitXor<U, Output = Self> { fn bitxor_assign(&mut self, rhs: U) { *self = self.clone().bitxor(rhs) }}
// impl<T, Inner, U> ShlAssign   <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: Clone + Shl   <U, Output = Self> { fn shl_assign   (&mut self, rhs: U) { *self = self.clone().shl   (rhs) }}
// impl<T, Inner, U> ShrAssign   <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: Clone + Shr   <U, Output = Self> { fn shr_assign   (&mut self, rhs: U) { *self = self.clone().shr   (rhs) }}

