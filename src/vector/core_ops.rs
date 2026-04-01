use core::ops::*;

use crate::{ops::*, static_list::StaticList, vector::Vector};


// Element-wise

impl<T, Inner   > Neg       for Vector<T, Inner> where Inner: StaticList<T>, Self: ElementNeg       { type Output = <Self as ElementNeg      >::Output; #[inline] fn neg   (self        ) -> Self::Output { self.element_neg   (   ) } }
impl<T, Inner   > Not       for Vector<T, Inner> where Inner: StaticList<T>, Self: ElementNot       { type Output = <Self as ElementNot      >::Output; #[inline] fn not   (self        ) -> Self::Output { self.element_not   (   ) } }
impl<T, Inner, U> Add   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: ElementAdd   <U> { type Output = <Self as ElementAdd   <U>>::Output; #[inline] fn add   (self, rhs: U) -> Self::Output { self.element_add   (rhs) } }
impl<T, Inner, U> Sub   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: ElementSub   <U> { type Output = <Self as ElementSub   <U>>::Output; #[inline] fn sub   (self, rhs: U) -> Self::Output { self.element_sub   (rhs) } }
impl<T, Inner, U> BitAnd<U> for Vector<T, Inner> where Inner: StaticList<T>, Self: ElementBitAnd<U> { type Output = <Self as ElementBitAnd<U>>::Output; #[inline] fn bitand(self, rhs: U) -> Self::Output { self.element_bitand(rhs) } }
impl<T, Inner, U> BitOr <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: ElementBitOr <U> { type Output = <Self as ElementBitOr <U>>::Output; #[inline] fn bitor (self, rhs: U) -> Self::Output { self.element_bitor (rhs) } }
impl<T, Inner, U> BitXor<U> for Vector<T, Inner> where Inner: StaticList<T>, Self: ElementBitXor<U> { type Output = <Self as ElementBitXor<U>>::Output; #[inline] fn bitxor(self, rhs: U) -> Self::Output { self.element_bitxor(rhs) } }
impl<T, Inner, U> Shl   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: ElementShl   <U> { type Output = <Self as ElementShl   <U>>::Output; #[inline] fn shl   (self, rhs: U) -> Self::Output { self.element_shl   (rhs) } }
impl<T, Inner, U> Shr   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: ElementShr   <U> { type Output = <Self as ElementShr   <U>>::Output; #[inline] fn shr   (self, rhs: U) -> Self::Output { self.element_shr   (rhs) } }

// Scalar

impl<T, Inner, U> Mul   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: ScalarMul    <U> { type Output = <Self as ScalarMul    <U>>::Output; #[inline] fn mul   (self, rhs: U) -> Self::Output { self.scalar_mul    (rhs) } }
impl<T, Inner, U> Div   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: ScalarDiv    <U> { type Output = <Self as ScalarDiv    <U>>::Output; #[inline] fn div   (self, rhs: U) -> Self::Output { self.scalar_div    (rhs) } }
impl<T, Inner, U> Rem   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: ScalarRem    <U> { type Output = <Self as ScalarRem    <U>>::Output; #[inline] fn rem   (self, rhs: U) -> Self::Output { self.scalar_rem    (rhs) } }

// Assign

impl<T, Inner, U> AddAssign   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: Clone + Add   <U, Output = Self> { fn add_assign   (&mut self, rhs: U) { *self = self.clone().add   (rhs) }}
impl<T, Inner, U> SubAssign   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: Clone + Sub   <U, Output = Self> { fn sub_assign   (&mut self, rhs: U) { *self = self.clone().sub   (rhs) }}
impl<T, Inner, U> MulAssign   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: Clone + Mul   <U, Output = Self> { fn mul_assign   (&mut self, rhs: U) { *self = self.clone().mul   (rhs) }}
impl<T, Inner, U> DivAssign   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: Clone + Div   <U, Output = Self> { fn div_assign   (&mut self, rhs: U) { *self = self.clone().div   (rhs) }}
impl<T, Inner, U> RemAssign   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: Clone + Rem   <U, Output = Self> { fn rem_assign   (&mut self, rhs: U) { *self = self.clone().rem   (rhs) }}
impl<T, Inner, U> BitAndAssign<U> for Vector<T, Inner> where Inner: StaticList<T>, Self: Clone + BitAnd<U, Output = Self> { fn bitand_assign(&mut self, rhs: U) { *self = self.clone().bitand(rhs) }}
impl<T, Inner, U> BitOrAssign <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: Clone + BitOr <U, Output = Self> { fn bitor_assign (&mut self, rhs: U) { *self = self.clone().bitor (rhs) }}
impl<T, Inner, U> BitXorAssign<U> for Vector<T, Inner> where Inner: StaticList<T>, Self: Clone + BitXor<U, Output = Self> { fn bitxor_assign(&mut self, rhs: U) { *self = self.clone().bitxor(rhs) }}
impl<T, Inner, U> ShlAssign   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: Clone + Shl   <U, Output = Self> { fn shl_assign   (&mut self, rhs: U) { *self = self.clone().shl   (rhs) }}
impl<T, Inner, U> ShrAssign   <U> for Vector<T, Inner> where Inner: StaticList<T>, Self: Clone + Shr   <U, Output = Self> { fn shr_assign   (&mut self, rhs: U) { *self = self.clone().shr   (rhs) }}

