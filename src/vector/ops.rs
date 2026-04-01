use core::ops::*;

use crate::{ops::*, static_list::StaticList, vector::{Vec0, Vector, helper::{Vec1, Vec3}}};


// Element-wise

impl<T   > ElementNeg             for Vec0<T> where T: Neg       { type Output = Vec0<<T as Neg      >::Output>; #[inline] fn element_neg   (self               ) -> Self::Output { Vec0::VALUE } }
impl<T   > ElementNot             for Vec0<T> where T: Not       { type Output = Vec0<<T as Not      >::Output>; #[inline] fn element_not   (self               ) -> Self::Output { Vec0::VALUE } }
impl<T, U> ElementAdd   <Vec0<U>> for Vec0<T> where T: Add   <U> { type Output = Vec0<<T as Add   <U>>::Output>; #[inline] fn element_add   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T, U> ElementSub   <Vec0<U>> for Vec0<T> where T: Sub   <U> { type Output = Vec0<<T as Sub   <U>>::Output>; #[inline] fn element_sub   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T, U> ElementMul   <Vec0<U>> for Vec0<T> where T: Mul   <U> { type Output = Vec0<<T as Mul   <U>>::Output>; #[inline] fn element_mul   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T, U> ElementDiv   <Vec0<U>> for Vec0<T> where T: Div   <U> { type Output = Vec0<<T as Div   <U>>::Output>; #[inline] fn element_div   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T, U> ElementRem   <Vec0<U>> for Vec0<T> where T: Rem   <U> { type Output = Vec0<<T as Rem   <U>>::Output>; #[inline] fn element_rem   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T, U> ElementBitAnd<Vec0<U>> for Vec0<T> where T: BitAnd<U> { type Output = Vec0<<T as BitAnd<U>>::Output>; #[inline] fn element_bitand(self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T, U> ElementBitOr <Vec0<U>> for Vec0<T> where T: BitOr <U> { type Output = Vec0<<T as BitOr <U>>::Output>; #[inline] fn element_bitor (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T, U> ElementBitXor<Vec0<U>> for Vec0<T> where T: BitXor<U> { type Output = Vec0<<T as BitXor<U>>::Output>; #[inline] fn element_bitxor(self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T, U> ElementShl   <Vec0<U>> for Vec0<T> where T: Shl   <U> { type Output = Vec0<<T as Shl   <U>>::Output>; #[inline] fn element_shl   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T, U> ElementShr   <Vec0<U>> for Vec0<T> where T: Shr   <U> { type Output = Vec0<<T as Shr   <U>>::Output>; #[inline] fn element_shr   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }

impl<T, Inner           > ElementNeg                       for Vector<T, Inner> where T: Neg      , Inner: StaticList<T> + ElementNeg           ,                        <Inner as ElementNeg           >::Output: StaticList<<T as Neg      >::Output> { type Output = Vector<<T as Neg      >::Output, <Inner as ElementNeg           >::Output>; #[inline] fn element_neg   (self                        ) -> Self::Output { Vector::push(self.inner.element_neg   (         ), self.end.neg   (       )) } }
impl<T, Inner           > ElementNot                       for Vector<T, Inner> where T: Not      , Inner: StaticList<T> + ElementNot           ,                        <Inner as ElementNot           >::Output: StaticList<<T as Not      >::Output> { type Output = Vector<<T as Not      >::Output, <Inner as ElementNot           >::Output>; #[inline] fn element_not   (self                        ) -> Self::Output { Vector::push(self.inner.element_not   (         ), self.end.not   (       )) } }
impl<T, Inner, U, UInner> ElementAdd   <Vector<U, UInner>> for Vector<T, Inner> where T: Add   <U>, Inner: StaticList<T> + ElementAdd   <UInner>, UInner: StaticList<U>, <Inner as ElementAdd   <UInner>>::Output: StaticList<<T as Add   <U>>::Output> { type Output = Vector<<T as Add   <U>>::Output, <Inner as ElementAdd   <UInner>>::Output>; #[inline] fn element_add   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector::push(self.inner.element_add   (rhs.inner), self.end.add   (rhs.end)) } }
impl<T, Inner, U, UInner> ElementSub   <Vector<U, UInner>> for Vector<T, Inner> where T: Sub   <U>, Inner: StaticList<T> + ElementSub   <UInner>, UInner: StaticList<U>, <Inner as ElementSub   <UInner>>::Output: StaticList<<T as Sub   <U>>::Output> { type Output = Vector<<T as Sub   <U>>::Output, <Inner as ElementSub   <UInner>>::Output>; #[inline] fn element_sub   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector::push(self.inner.element_sub   (rhs.inner), self.end.sub   (rhs.end)) } }
impl<T, Inner, U, UInner> ElementMul   <Vector<U, UInner>> for Vector<T, Inner> where T: Mul   <U>, Inner: StaticList<T> + ElementMul   <UInner>, UInner: StaticList<U>, <Inner as ElementMul   <UInner>>::Output: StaticList<<T as Mul   <U>>::Output> { type Output = Vector<<T as Mul   <U>>::Output, <Inner as ElementMul   <UInner>>::Output>; #[inline] fn element_mul   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector::push(self.inner.element_mul   (rhs.inner), self.end.mul   (rhs.end)) } }
impl<T, Inner, U, UInner> ElementDiv   <Vector<U, UInner>> for Vector<T, Inner> where T: Div   <U>, Inner: StaticList<T> + ElementDiv   <UInner>, UInner: StaticList<U>, <Inner as ElementDiv   <UInner>>::Output: StaticList<<T as Div   <U>>::Output> { type Output = Vector<<T as Div   <U>>::Output, <Inner as ElementDiv   <UInner>>::Output>; #[inline] fn element_div   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector::push(self.inner.element_div   (rhs.inner), self.end.div   (rhs.end)) } }
impl<T, Inner, U, UInner> ElementRem   <Vector<U, UInner>> for Vector<T, Inner> where T: Rem   <U>, Inner: StaticList<T> + ElementRem   <UInner>, UInner: StaticList<U>, <Inner as ElementRem   <UInner>>::Output: StaticList<<T as Rem   <U>>::Output> { type Output = Vector<<T as Rem   <U>>::Output, <Inner as ElementRem   <UInner>>::Output>; #[inline] fn element_rem   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector::push(self.inner.element_rem   (rhs.inner), self.end.rem   (rhs.end)) } }
impl<T, Inner, U, UInner> ElementBitAnd<Vector<U, UInner>> for Vector<T, Inner> where T: BitAnd<U>, Inner: StaticList<T> + ElementBitAnd<UInner>, UInner: StaticList<U>, <Inner as ElementBitAnd<UInner>>::Output: StaticList<<T as BitAnd<U>>::Output> { type Output = Vector<<T as BitAnd<U>>::Output, <Inner as ElementBitAnd<UInner>>::Output>; #[inline] fn element_bitand(self, rhs: Vector<U, UInner>) -> Self::Output { Vector::push(self.inner.element_bitand(rhs.inner), self.end.bitand(rhs.end)) } }
impl<T, Inner, U, UInner> ElementBitOr <Vector<U, UInner>> for Vector<T, Inner> where T: BitOr <U>, Inner: StaticList<T> + ElementBitOr <UInner>, UInner: StaticList<U>, <Inner as ElementBitOr <UInner>>::Output: StaticList<<T as BitOr <U>>::Output> { type Output = Vector<<T as BitOr <U>>::Output, <Inner as ElementBitOr <UInner>>::Output>; #[inline] fn element_bitor (self, rhs: Vector<U, UInner>) -> Self::Output { Vector::push(self.inner.element_bitor (rhs.inner), self.end.bitor (rhs.end)) } }
impl<T, Inner, U, UInner> ElementBitXor<Vector<U, UInner>> for Vector<T, Inner> where T: BitXor<U>, Inner: StaticList<T> + ElementBitXor<UInner>, UInner: StaticList<U>, <Inner as ElementBitXor<UInner>>::Output: StaticList<<T as BitXor<U>>::Output> { type Output = Vector<<T as BitXor<U>>::Output, <Inner as ElementBitXor<UInner>>::Output>; #[inline] fn element_bitxor(self, rhs: Vector<U, UInner>) -> Self::Output { Vector::push(self.inner.element_bitxor(rhs.inner), self.end.bitxor(rhs.end)) } }
impl<T, Inner, U, UInner> ElementShl   <Vector<U, UInner>> for Vector<T, Inner> where T: Shl   <U>, Inner: StaticList<T> + ElementShl   <UInner>, UInner: StaticList<U>, <Inner as ElementShl   <UInner>>::Output: StaticList<<T as Shl   <U>>::Output> { type Output = Vector<<T as Shl   <U>>::Output, <Inner as ElementShl   <UInner>>::Output>; #[inline] fn element_shl   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector::push(self.inner.element_shl   (rhs.inner), self.end.shl   (rhs.end)) } }
impl<T, Inner, U, UInner> ElementShr   <Vector<U, UInner>> for Vector<T, Inner> where T: Shr   <U>, Inner: StaticList<T> + ElementShr   <UInner>, UInner: StaticList<U>, <Inner as ElementShr   <UInner>>::Output: StaticList<<T as Shr   <U>>::Output> { type Output = Vector<<T as Shr   <U>>::Output, <Inner as ElementShr   <UInner>>::Output>; #[inline] fn element_shr   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector::push(self.inner.element_shr   (rhs.inner), self.end.shr   (rhs.end)) } }


// Scalar

impl<T, U> ScalarMul   <U> for Vec0<T> where T: Mul   <U> { type Output = Vec0<<T as Mul   <U>>::Output>; #[inline] fn scalar_mul   (self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T, U> ScalarDiv   <U> for Vec0<T> where T: Div   <U> { type Output = Vec0<<T as Div   <U>>::Output>; #[inline] fn scalar_div   (self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T, U> ScalarRem   <U> for Vec0<T> where T: Rem   <U> { type Output = Vec0<<T as Rem   <U>>::Output>; #[inline] fn scalar_rem   (self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T, U> ScalarBitAnd<U> for Vec0<T> where T: BitAnd<U> { type Output = Vec0<<T as BitAnd<U>>::Output>; #[inline] fn scalar_bitand(self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T, U> ScalarBitOr <U> for Vec0<T> where T: BitOr <U> { type Output = Vec0<<T as BitOr <U>>::Output>; #[inline] fn scalar_bitor (self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T, U> ScalarBitXor<U> for Vec0<T> where T: BitXor<U> { type Output = Vec0<<T as BitXor<U>>::Output>; #[inline] fn scalar_bitxor(self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T, U> ScalarShl   <U> for Vec0<T> where T: Shl   <U> { type Output = Vec0<<T as Shl   <U>>::Output>; #[inline] fn scalar_shl   (self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T, U> ScalarShr   <U> for Vec0<T> where T: Shr   <U> { type Output = Vec0<<T as Shr   <U>>::Output>; #[inline] fn scalar_shr   (self, _rhs: U) -> Self::Output { Vec0::VALUE } }

impl<T, Inner, U> ScalarMul   <U> for Vector<T, Inner> where T: Mul   <U>, Inner: StaticList<T> + ScalarMul   <U>, U: Clone, <Inner as ScalarMul   <U>>::Output: StaticList<<T as Mul   <U>>::Output> { type Output = Vector<<T as Mul   <U>>::Output, <Inner as ScalarMul   <U>>::Output>; #[inline] fn scalar_mul   (self, rhs: U) -> Self::Output { Vector::push(self.inner.scalar_mul   (rhs.clone()), self.end.mul   (rhs)) } }
impl<T, Inner, U> ScalarDiv   <U> for Vector<T, Inner> where T: Div   <U>, Inner: StaticList<T> + ScalarDiv   <U>, U: Clone, <Inner as ScalarDiv   <U>>::Output: StaticList<<T as Div   <U>>::Output> { type Output = Vector<<T as Div   <U>>::Output, <Inner as ScalarDiv   <U>>::Output>; #[inline] fn scalar_div   (self, rhs: U) -> Self::Output { Vector::push(self.inner.scalar_div   (rhs.clone()), self.end.div   (rhs)) } }
impl<T, Inner, U> ScalarRem   <U> for Vector<T, Inner> where T: Rem   <U>, Inner: StaticList<T> + ScalarRem   <U>, U: Clone, <Inner as ScalarRem   <U>>::Output: StaticList<<T as Rem   <U>>::Output> { type Output = Vector<<T as Rem   <U>>::Output, <Inner as ScalarRem   <U>>::Output>; #[inline] fn scalar_rem   (self, rhs: U) -> Self::Output { Vector::push(self.inner.scalar_rem   (rhs.clone()), self.end.rem   (rhs)) } }
impl<T, Inner, U> ScalarBitAnd<U> for Vector<T, Inner> where T: BitAnd<U>, Inner: StaticList<T> + ScalarBitAnd<U>, U: Clone, <Inner as ScalarBitAnd<U>>::Output: StaticList<<T as BitAnd<U>>::Output> { type Output = Vector<<T as BitAnd<U>>::Output, <Inner as ScalarBitAnd<U>>::Output>; #[inline] fn scalar_bitand(self, rhs: U) -> Self::Output { Vector::push(self.inner.scalar_bitand(rhs.clone()), self.end.bitand(rhs)) } }
impl<T, Inner, U> ScalarBitOr <U> for Vector<T, Inner> where T: BitOr <U>, Inner: StaticList<T> + ScalarBitOr <U>, U: Clone, <Inner as ScalarBitOr <U>>::Output: StaticList<<T as BitOr <U>>::Output> { type Output = Vector<<T as BitOr <U>>::Output, <Inner as ScalarBitOr <U>>::Output>; #[inline] fn scalar_bitor (self, rhs: U) -> Self::Output { Vector::push(self.inner.scalar_bitor (rhs.clone()), self.end.bitor (rhs)) } }
impl<T, Inner, U> ScalarBitXor<U> for Vector<T, Inner> where T: BitXor<U>, Inner: StaticList<T> + ScalarBitXor<U>, U: Clone, <Inner as ScalarBitXor<U>>::Output: StaticList<<T as BitXor<U>>::Output> { type Output = Vector<<T as BitXor<U>>::Output, <Inner as ScalarBitXor<U>>::Output>; #[inline] fn scalar_bitxor(self, rhs: U) -> Self::Output { Vector::push(self.inner.scalar_bitxor(rhs.clone()), self.end.bitxor(rhs)) } }
impl<T, Inner, U> ScalarShl   <U> for Vector<T, Inner> where T: Shl   <U>, Inner: StaticList<T> + ScalarShl   <U>, U: Clone, <Inner as ScalarShl   <U>>::Output: StaticList<<T as Shl   <U>>::Output> { type Output = Vector<<T as Shl   <U>>::Output, <Inner as ScalarShl   <U>>::Output>; #[inline] fn scalar_shl   (self, rhs: U) -> Self::Output { Vector::push(self.inner.scalar_shl   (rhs.clone()), self.end.shl   (rhs)) } }
impl<T, Inner, U> ScalarShr   <U> for Vector<T, Inner> where T: Shr   <U>, Inner: StaticList<T> + ScalarShr   <U>, U: Clone, <Inner as ScalarShr   <U>>::Output: StaticList<<T as Shr   <U>>::Output> { type Output = Vector<<T as Shr   <U>>::Output, <Inner as ScalarShr   <U>>::Output>; #[inline] fn scalar_shr   (self, rhs: U) -> Self::Output { Vector::push(self.inner.scalar_shr   (rhs.clone()), self.end.shr   (rhs)) } }


// Custom

impl<T, U> Dot<Vec1<U>> for Vec1<T>
where
    T: Mul<U>,
{
    type Output = <T as Mul<U>>::Output;
    #[inline]
    fn dot(self, rhs: Vec1<U>) -> Self::Output {
        self.end * rhs.end
    }
}

impl<T, Inner, U, UInner> Dot<Vector<U, Vector<U, UInner>>> for Vector<T, Vector<T, Inner>>
where
    T: Mul<U>,
    Inner: StaticList<T>,
    UInner: StaticList<U>,
    Vector<T, Inner>: Dot<Vector<U, UInner>>,
    <Vector<T, Inner> as Dot<Vector<U, UInner>>>::Output: Add<<T as Mul<U>>::Output>,
{
    type Output = <<Vector<T, Inner> as Dot<Vector<U, UInner>>>::Output as Add<<T as Mul<U>>::Output>>::Output;
    #[inline]
    fn dot(self, rhs: Vector<U, Vector<U, UInner>>) -> Self::Output {
        self.inner.dot(rhs.inner) + self.end * rhs.end
    }
}


impl<T, U> Cross<Vec3<U>> for Vec3<T>
where
    T: Clone + Mul<U>,
    U: Clone,
    <T as Mul<U>>::Output: Sub,
{
    type Output = Vec3<<<T as Mul<U>>::Output as Sub>::Output>;
    #[inline]
    fn cross(self, rhs: Vec3<U>) -> Self::Output {
        Vec3::new(
            self.clone().y() * rhs.clone().z() - self.clone().z() * rhs.clone().y(),
            self.clone().z() * rhs.clone().x() - self.clone().x() * rhs.clone().z(),
            self.clone().x() * rhs.clone().y() - self.clone().y() * rhs.clone().x(),
        )
    }
}

