use core::ops::*;

use crate::{static_list::{StaticMap, StaticZip}, vector::{VecPush, Vector}};


// Element-wise

impl<T, Inner> Neg for VecPush<T, Inner> where Inner: Vector<T>, Self: StaticMap<T, T::Output>, T: Neg { type Output = <Self as StaticMap<T, T::Output>>::MapOutput; #[inline] fn neg(self) -> Self::Output { self.map(&Neg::neg) } }
impl<T, Inner> Not for VecPush<T, Inner> where Inner: Vector<T>, Self: StaticMap<T, T::Output>, T: Not { type Output = <Self as StaticMap<T, T::Output>>::MapOutput; #[inline] fn not(self) -> Self::Output { self.map(&Not::not) } }

impl<T, Inner, R, RInner> Add   <VecPush<R, RInner>> for VecPush<T, Inner> where Inner: Vector<T> + StaticZip<T, T::Output, R, RInner>, RInner: Vector<R>, Inner::ZipOutput: Vector<T::Output>, Self: StaticZip<T, T::Output, R, VecPush<R, RInner>>, T: Add   <R> { type Output = <Self as StaticZip<T, T::Output, R, VecPush<R, RInner>>>::ZipOutput; #[inline] fn add   (self, rhs: VecPush<R, RInner>) -> Self::Output { self.zip(rhs, &Add   ::add   ) } }
impl<T, Inner, R, RInner> Sub   <VecPush<R, RInner>> for VecPush<T, Inner> where Inner: Vector<T> + StaticZip<T, T::Output, R, RInner>, RInner: Vector<R>, Inner::ZipOutput: Vector<T::Output>, Self: StaticZip<T, T::Output, R, VecPush<R, RInner>>, T: Sub   <R> { type Output = <Self as StaticZip<T, T::Output, R, VecPush<R, RInner>>>::ZipOutput; #[inline] fn sub   (self, rhs: VecPush<R, RInner>) -> Self::Output { self.zip(rhs, &Sub   ::sub   ) } }
impl<T, Inner, R, RInner> BitAnd<VecPush<R, RInner>> for VecPush<T, Inner> where Inner: Vector<T> + StaticZip<T, T::Output, R, RInner>, RInner: Vector<R>, Inner::ZipOutput: Vector<T::Output>, Self: StaticZip<T, T::Output, R, VecPush<R, RInner>>, T: BitAnd<R> { type Output = <Self as StaticZip<T, T::Output, R, VecPush<R, RInner>>>::ZipOutput; #[inline] fn bitand(self, rhs: VecPush<R, RInner>) -> Self::Output { self.zip(rhs, &BitAnd::bitand) } }
impl<T, Inner, R, RInner> BitOr <VecPush<R, RInner>> for VecPush<T, Inner> where Inner: Vector<T> + StaticZip<T, T::Output, R, RInner>, RInner: Vector<R>, Inner::ZipOutput: Vector<T::Output>, Self: StaticZip<T, T::Output, R, VecPush<R, RInner>>, T: BitOr <R> { type Output = <Self as StaticZip<T, T::Output, R, VecPush<R, RInner>>>::ZipOutput; #[inline] fn bitor (self, rhs: VecPush<R, RInner>) -> Self::Output { self.zip(rhs, &BitOr ::bitor ) } }
impl<T, Inner, R, RInner> BitXor<VecPush<R, RInner>> for VecPush<T, Inner> where Inner: Vector<T> + StaticZip<T, T::Output, R, RInner>, RInner: Vector<R>, Inner::ZipOutput: Vector<T::Output>, Self: StaticZip<T, T::Output, R, VecPush<R, RInner>>, T: BitXor<R> { type Output = <Self as StaticZip<T, T::Output, R, VecPush<R, RInner>>>::ZipOutput; #[inline] fn bitxor(self, rhs: VecPush<R, RInner>) -> Self::Output { self.zip(rhs, &BitXor::bitxor) } }
impl<T, Inner, R, RInner> Shl   <VecPush<R, RInner>> for VecPush<T, Inner> where Inner: Vector<T> + StaticZip<T, T::Output, R, RInner>, RInner: Vector<R>, Inner::ZipOutput: Vector<T::Output>, Self: StaticZip<T, T::Output, R, VecPush<R, RInner>>, T: Shl   <R> { type Output = <Self as StaticZip<T, T::Output, R, VecPush<R, RInner>>>::ZipOutput; #[inline] fn shl   (self, rhs: VecPush<R, RInner>) -> Self::Output { self.zip(rhs, &Shl   ::shl   ) } }
impl<T, Inner, R, RInner> Shr   <VecPush<R, RInner>> for VecPush<T, Inner> where Inner: Vector<T> + StaticZip<T, T::Output, R, RInner>, RInner: Vector<R>, Inner::ZipOutput: Vector<T::Output>, Self: StaticZip<T, T::Output, R, VecPush<R, RInner>>, T: Shr   <R> { type Output = <Self as StaticZip<T, T::Output, R, VecPush<R, RInner>>>::ZipOutput; #[inline] fn shr   (self, rhs: VecPush<R, RInner>) -> Self::Output { self.zip(rhs, &Shr   ::shr   ) } }


// Scalar

// impl<T, Inner, U> Mul   <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: ScalarMul    <U> { type Output = <Self as ScalarMul    <U>>::Output; #[inline] fn mul   (self, rhs: U) -> Self::Output { self.scalar_mul    (rhs) } }
// impl<T, Inner, U> Div   <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: ScalarDiv    <U> { type Output = <Self as ScalarDiv    <U>>::Output; #[inline] fn div   (self, rhs: U) -> Self::Output { self.scalar_div    (rhs) } }
// impl<T, Inner, U> Rem   <U> for VecPush<T, Inner> where Inner: StaticList<T>, Self: ScalarRem    <U> { type Output = <Self as ScalarRem    <U>>::Output; #[inline] fn rem   (self, rhs: U) -> Self::Output { self.scalar_rem    (rhs) } }

// Assign

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

