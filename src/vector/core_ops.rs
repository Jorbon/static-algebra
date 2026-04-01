use core::ops::*;

use crate::{ops::*, static_list::StaticList, vector::Vector};


// Element-wise

impl<T, Inner: StaticList<T>   > Neg       for Vector<T, Inner> where Self: ElementNeg       { type Output = <Self as ElementNeg      >::Output; #[inline] fn neg   (self        ) -> Self::Output { self.element_neg   (   ) } }
impl<T, Inner: StaticList<T>   > Not       for Vector<T, Inner> where Self: ElementNot       { type Output = <Self as ElementNot      >::Output; #[inline] fn not   (self        ) -> Self::Output { self.element_not   (   ) } }
impl<T, Inner: StaticList<T>, U> Add   <U> for Vector<T, Inner> where Self: ElementAdd   <U> { type Output = <Self as ElementAdd   <U>>::Output; #[inline] fn add   (self, rhs: U) -> Self::Output { self.element_add   (rhs) } }
impl<T, Inner: StaticList<T>, U> Sub   <U> for Vector<T, Inner> where Self: ElementSub   <U> { type Output = <Self as ElementSub   <U>>::Output; #[inline] fn sub   (self, rhs: U) -> Self::Output { self.element_sub   (rhs) } }
impl<T, Inner: StaticList<T>, U> BitAnd<U> for Vector<T, Inner> where Self: ElementBitAnd<U> { type Output = <Self as ElementBitAnd<U>>::Output; #[inline] fn bitand(self, rhs: U) -> Self::Output { self.element_bitand(rhs) } }
impl<T, Inner: StaticList<T>, U> BitOr <U> for Vector<T, Inner> where Self: ElementBitOr <U> { type Output = <Self as ElementBitOr <U>>::Output; #[inline] fn bitor (self, rhs: U) -> Self::Output { self.element_bitor (rhs) } }
impl<T, Inner: StaticList<T>, U> BitXor<U> for Vector<T, Inner> where Self: ElementBitXor<U> { type Output = <Self as ElementBitXor<U>>::Output; #[inline] fn bitxor(self, rhs: U) -> Self::Output { self.element_bitxor(rhs) } }
impl<T, Inner: StaticList<T>, U> Shl   <U> for Vector<T, Inner> where Self: ElementShl   <U> { type Output = <Self as ElementShl   <U>>::Output; #[inline] fn shl   (self, rhs: U) -> Self::Output { self.element_shl   (rhs) } }
impl<T, Inner: StaticList<T>, U> Shr   <U> for Vector<T, Inner> where Self: ElementShr   <U> { type Output = <Self as ElementShr   <U>>::Output; #[inline] fn shr   (self, rhs: U) -> Self::Output { self.element_shr   (rhs) } }

// Scalar

impl<T, Inner: StaticList<T>, U> Mul   <U> for Vector<T, Inner> where Self: ScalarMul    <U> { type Output = <Self as ScalarMul    <U>>::Output; #[inline] fn mul   (self, rhs: U) -> Self::Output { self.scalar_mul    (rhs) } }
impl<T, Inner: StaticList<T>, U> Div   <U> for Vector<T, Inner> where Self: ScalarDiv    <U> { type Output = <Self as ScalarDiv    <U>>::Output; #[inline] fn div   (self, rhs: U) -> Self::Output { self.scalar_div    (rhs) } }
impl<T, Inner: StaticList<T>, U> Rem   <U> for Vector<T, Inner> where Self: ScalarRem    <U> { type Output = <Self as ScalarRem    <U>>::Output; #[inline] fn rem   (self, rhs: U) -> Self::Output { self.scalar_rem    (rhs) } }

// Assign

impl<T, Inner: StaticList<T>, U> AddAssign   <U> for Vector<T, Inner> where Self: Clone + Add   <U, Output = Self> { fn add_assign   (&mut self, rhs: U) { *self = self.clone().add   (rhs) }}
impl<T, Inner: StaticList<T>, U> SubAssign   <U> for Vector<T, Inner> where Self: Clone + Sub   <U, Output = Self> { fn sub_assign   (&mut self, rhs: U) { *self = self.clone().sub   (rhs) }}
impl<T, Inner: StaticList<T>, U> MulAssign   <U> for Vector<T, Inner> where Self: Clone + Mul   <U, Output = Self> { fn mul_assign   (&mut self, rhs: U) { *self = self.clone().mul   (rhs) }}
impl<T, Inner: StaticList<T>, U> DivAssign   <U> for Vector<T, Inner> where Self: Clone + Div   <U, Output = Self> { fn div_assign   (&mut self, rhs: U) { *self = self.clone().div   (rhs) }}
impl<T, Inner: StaticList<T>, U> RemAssign   <U> for Vector<T, Inner> where Self: Clone + Rem   <U, Output = Self> { fn rem_assign   (&mut self, rhs: U) { *self = self.clone().rem   (rhs) }}
impl<T, Inner: StaticList<T>, U> BitAndAssign<U> for Vector<T, Inner> where Self: Clone + BitAnd<U, Output = Self> { fn bitand_assign(&mut self, rhs: U) { *self = self.clone().bitand(rhs) }}
impl<T, Inner: StaticList<T>, U> BitOrAssign <U> for Vector<T, Inner> where Self: Clone + BitOr <U, Output = Self> { fn bitor_assign (&mut self, rhs: U) { *self = self.clone().bitor (rhs) }}
impl<T, Inner: StaticList<T>, U> BitXorAssign<U> for Vector<T, Inner> where Self: Clone + BitXor<U, Output = Self> { fn bitxor_assign(&mut self, rhs: U) { *self = self.clone().bitxor(rhs) }}
impl<T, Inner: StaticList<T>, U> ShlAssign   <U> for Vector<T, Inner> where Self: Clone + Shl   <U, Output = Self> { fn shl_assign   (&mut self, rhs: U) { *self = self.clone().shl   (rhs) }}
impl<T, Inner: StaticList<T>, U> ShrAssign   <U> for Vector<T, Inner> where Self: Clone + Shr   <U, Output = Self> { fn shr_assign   (&mut self, rhs: U) { *self = self.clone().shr   (rhs) }}

