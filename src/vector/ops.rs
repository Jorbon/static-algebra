use core::ops::*;

use crate::{ops::*, static_list::StaticList, vector::{Vec0, Vector, helper::{Vec1, Vec3}}};


// Element-wise

impl<T: Neg         > ElementNeg             for Vec0<T> { type Output = Vec0<<T as Neg      >::Output>; #[inline] fn element_neg   (self               ) -> Self::Output { Vec0::VALUE } }
impl<T: Not         > ElementNot             for Vec0<T> { type Output = Vec0<<T as Not      >::Output>; #[inline] fn element_not   (self               ) -> Self::Output { Vec0::VALUE } }
impl<T: Add   <U>, U> ElementAdd   <Vec0<U>> for Vec0<T> { type Output = Vec0<<T as Add   <U>>::Output>; #[inline] fn element_add   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T: Sub   <U>, U> ElementSub   <Vec0<U>> for Vec0<T> { type Output = Vec0<<T as Sub   <U>>::Output>; #[inline] fn element_sub   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T: Mul   <U>, U> ElementMul   <Vec0<U>> for Vec0<T> { type Output = Vec0<<T as Mul   <U>>::Output>; #[inline] fn element_mul   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T: Div   <U>, U> ElementDiv   <Vec0<U>> for Vec0<T> { type Output = Vec0<<T as Div   <U>>::Output>; #[inline] fn element_div   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T: Rem   <U>, U> ElementRem   <Vec0<U>> for Vec0<T> { type Output = Vec0<<T as Rem   <U>>::Output>; #[inline] fn element_rem   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T: BitAnd<U>, U> ElementBitAnd<Vec0<U>> for Vec0<T> { type Output = Vec0<<T as BitAnd<U>>::Output>; #[inline] fn element_bitand(self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T: BitOr <U>, U> ElementBitOr <Vec0<U>> for Vec0<T> { type Output = Vec0<<T as BitOr <U>>::Output>; #[inline] fn element_bitor (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T: BitXor<U>, U> ElementBitXor<Vec0<U>> for Vec0<T> { type Output = Vec0<<T as BitXor<U>>::Output>; #[inline] fn element_bitxor(self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T: Shl   <U>, U> ElementShl   <Vec0<U>> for Vec0<T> { type Output = Vec0<<T as Shl   <U>>::Output>; #[inline] fn element_shl   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }
impl<T: Shr   <U>, U> ElementShr   <Vec0<U>> for Vec0<T> { type Output = Vec0<<T as Shr   <U>>::Output>; #[inline] fn element_shr   (self, _rhs: Vec0<U>) -> Self::Output { Vec0::VALUE } }

impl<T: Neg      , Inner: StaticList<T> + ElementNeg          >                            ElementNeg                       for Vector<T, Inner> where <Inner as ElementNeg           >::Output: StaticList<<T as Neg      >::Output> { type Output = Vector<<T as Neg      >::Output, <Inner as ElementNeg           >::Output>; #[inline] fn element_neg   (self                        ) -> Self::Output { Vector(self.0.element_neg   (     ), self.1.neg   (     )) } }
impl<T: Not      , Inner: StaticList<T> + ElementNot          >                            ElementNot                       for Vector<T, Inner> where <Inner as ElementNot           >::Output: StaticList<<T as Not      >::Output> { type Output = Vector<<T as Not      >::Output, <Inner as ElementNot           >::Output>; #[inline] fn element_not   (self                        ) -> Self::Output { Vector(self.0.element_not   (     ), self.1.not   (     )) } }
impl<T: Add   <U>, Inner: StaticList<T> + ElementAdd   <UInner>, U, UInner: StaticList<U>> ElementAdd   <Vector<U, UInner>> for Vector<T, Inner> where <Inner as ElementAdd   <UInner>>::Output: StaticList<<T as Add   <U>>::Output> { type Output = Vector<<T as Add   <U>>::Output, <Inner as ElementAdd   <UInner>>::Output>; #[inline] fn element_add   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector(self.0.element_add   (rhs.0), self.1.add   (rhs.1)) } }
impl<T: Sub   <U>, Inner: StaticList<T> + ElementSub   <UInner>, U, UInner: StaticList<U>> ElementSub   <Vector<U, UInner>> for Vector<T, Inner> where <Inner as ElementSub   <UInner>>::Output: StaticList<<T as Sub   <U>>::Output> { type Output = Vector<<T as Sub   <U>>::Output, <Inner as ElementSub   <UInner>>::Output>; #[inline] fn element_sub   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector(self.0.element_sub   (rhs.0), self.1.sub   (rhs.1)) } }
impl<T: Mul   <U>, Inner: StaticList<T> + ElementMul   <UInner>, U, UInner: StaticList<U>> ElementMul   <Vector<U, UInner>> for Vector<T, Inner> where <Inner as ElementMul   <UInner>>::Output: StaticList<<T as Mul   <U>>::Output> { type Output = Vector<<T as Mul   <U>>::Output, <Inner as ElementMul   <UInner>>::Output>; #[inline] fn element_mul   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector(self.0.element_mul   (rhs.0), self.1.mul   (rhs.1)) } }
impl<T: Div   <U>, Inner: StaticList<T> + ElementDiv   <UInner>, U, UInner: StaticList<U>> ElementDiv   <Vector<U, UInner>> for Vector<T, Inner> where <Inner as ElementDiv   <UInner>>::Output: StaticList<<T as Div   <U>>::Output> { type Output = Vector<<T as Div   <U>>::Output, <Inner as ElementDiv   <UInner>>::Output>; #[inline] fn element_div   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector(self.0.element_div   (rhs.0), self.1.div   (rhs.1)) } }
impl<T: Rem   <U>, Inner: StaticList<T> + ElementRem   <UInner>, U, UInner: StaticList<U>> ElementRem   <Vector<U, UInner>> for Vector<T, Inner> where <Inner as ElementRem   <UInner>>::Output: StaticList<<T as Rem   <U>>::Output> { type Output = Vector<<T as Rem   <U>>::Output, <Inner as ElementRem   <UInner>>::Output>; #[inline] fn element_rem   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector(self.0.element_rem   (rhs.0), self.1.rem   (rhs.1)) } }
impl<T: BitAnd<U>, Inner: StaticList<T> + ElementBitAnd<UInner>, U, UInner: StaticList<U>> ElementBitAnd<Vector<U, UInner>> for Vector<T, Inner> where <Inner as ElementBitAnd<UInner>>::Output: StaticList<<T as BitAnd<U>>::Output> { type Output = Vector<<T as BitAnd<U>>::Output, <Inner as ElementBitAnd<UInner>>::Output>; #[inline] fn element_bitand(self, rhs: Vector<U, UInner>) -> Self::Output { Vector(self.0.element_bitand(rhs.0), self.1.bitand(rhs.1)) } }
impl<T: BitOr <U>, Inner: StaticList<T> + ElementBitOr <UInner>, U, UInner: StaticList<U>> ElementBitOr <Vector<U, UInner>> for Vector<T, Inner> where <Inner as ElementBitOr <UInner>>::Output: StaticList<<T as BitOr <U>>::Output> { type Output = Vector<<T as BitOr <U>>::Output, <Inner as ElementBitOr <UInner>>::Output>; #[inline] fn element_bitor (self, rhs: Vector<U, UInner>) -> Self::Output { Vector(self.0.element_bitor (rhs.0), self.1.bitor (rhs.1)) } }
impl<T: BitXor<U>, Inner: StaticList<T> + ElementBitXor<UInner>, U, UInner: StaticList<U>> ElementBitXor<Vector<U, UInner>> for Vector<T, Inner> where <Inner as ElementBitXor<UInner>>::Output: StaticList<<T as BitXor<U>>::Output> { type Output = Vector<<T as BitXor<U>>::Output, <Inner as ElementBitXor<UInner>>::Output>; #[inline] fn element_bitxor(self, rhs: Vector<U, UInner>) -> Self::Output { Vector(self.0.element_bitxor(rhs.0), self.1.bitxor(rhs.1)) } }
impl<T: Shl   <U>, Inner: StaticList<T> + ElementShl   <UInner>, U, UInner: StaticList<U>> ElementShl   <Vector<U, UInner>> for Vector<T, Inner> where <Inner as ElementShl   <UInner>>::Output: StaticList<<T as Shl   <U>>::Output> { type Output = Vector<<T as Shl   <U>>::Output, <Inner as ElementShl   <UInner>>::Output>; #[inline] fn element_shl   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector(self.0.element_shl   (rhs.0), self.1.shl   (rhs.1)) } }
impl<T: Shr   <U>, Inner: StaticList<T> + ElementShr   <UInner>, U, UInner: StaticList<U>> ElementShr   <Vector<U, UInner>> for Vector<T, Inner> where <Inner as ElementShr   <UInner>>::Output: StaticList<<T as Shr   <U>>::Output> { type Output = Vector<<T as Shr   <U>>::Output, <Inner as ElementShr   <UInner>>::Output>; #[inline] fn element_shr   (self, rhs: Vector<U, UInner>) -> Self::Output { Vector(self.0.element_shr   (rhs.0), self.1.shr   (rhs.1)) } }


// Scalar

impl<T: Mul   <U>, U> ScalarMul   <U> for Vec0<T> { type Output = Vec0<<T as Mul   <U>>::Output>; #[inline] fn scalar_mul   (self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T: Div   <U>, U> ScalarDiv   <U> for Vec0<T> { type Output = Vec0<<T as Div   <U>>::Output>; #[inline] fn scalar_div   (self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T: Rem   <U>, U> ScalarRem   <U> for Vec0<T> { type Output = Vec0<<T as Rem   <U>>::Output>; #[inline] fn scalar_rem   (self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T: BitAnd<U>, U> ScalarBitAnd<U> for Vec0<T> { type Output = Vec0<<T as BitAnd<U>>::Output>; #[inline] fn scalar_bitand(self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T: BitOr <U>, U> ScalarBitOr <U> for Vec0<T> { type Output = Vec0<<T as BitOr <U>>::Output>; #[inline] fn scalar_bitor (self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T: BitXor<U>, U> ScalarBitXor<U> for Vec0<T> { type Output = Vec0<<T as BitXor<U>>::Output>; #[inline] fn scalar_bitxor(self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T: Shl   <U>, U> ScalarShl   <U> for Vec0<T> { type Output = Vec0<<T as Shl   <U>>::Output>; #[inline] fn scalar_shl   (self, _rhs: U) -> Self::Output { Vec0::VALUE } }
impl<T: Shr   <U>, U> ScalarShr   <U> for Vec0<T> { type Output = Vec0<<T as Shr   <U>>::Output>; #[inline] fn scalar_shr   (self, _rhs: U) -> Self::Output { Vec0::VALUE } }

impl<T: Mul   <U>, Inner: StaticList<T> + ScalarMul   <U>, U: Clone> ScalarMul   <U> for Vector<T, Inner> where <Inner as ScalarMul   <U>>::Output: StaticList<<T as Mul   <U>>::Output> { type Output = Vector<<T as Mul   <U>>::Output, <Inner as ScalarMul   <U>>::Output>; #[inline] fn scalar_mul   (self, rhs: U) -> Self::Output { Vector(self.0.scalar_mul   (rhs.clone()), self.1.mul   (rhs)) } }
impl<T: Div   <U>, Inner: StaticList<T> + ScalarDiv   <U>, U: Clone> ScalarDiv   <U> for Vector<T, Inner> where <Inner as ScalarDiv   <U>>::Output: StaticList<<T as Div   <U>>::Output> { type Output = Vector<<T as Div   <U>>::Output, <Inner as ScalarDiv   <U>>::Output>; #[inline] fn scalar_div   (self, rhs: U) -> Self::Output { Vector(self.0.scalar_div   (rhs.clone()), self.1.div   (rhs)) } }
impl<T: Rem   <U>, Inner: StaticList<T> + ScalarRem   <U>, U: Clone> ScalarRem   <U> for Vector<T, Inner> where <Inner as ScalarRem   <U>>::Output: StaticList<<T as Rem   <U>>::Output> { type Output = Vector<<T as Rem   <U>>::Output, <Inner as ScalarRem   <U>>::Output>; #[inline] fn scalar_rem   (self, rhs: U) -> Self::Output { Vector(self.0.scalar_rem   (rhs.clone()), self.1.rem   (rhs)) } }
impl<T: BitAnd<U>, Inner: StaticList<T> + ScalarBitAnd<U>, U: Clone> ScalarBitAnd<U> for Vector<T, Inner> where <Inner as ScalarBitAnd<U>>::Output: StaticList<<T as BitAnd<U>>::Output> { type Output = Vector<<T as BitAnd<U>>::Output, <Inner as ScalarBitAnd<U>>::Output>; #[inline] fn scalar_bitand(self, rhs: U) -> Self::Output { Vector(self.0.scalar_bitand(rhs.clone()), self.1.bitand(rhs)) } }
impl<T: BitOr <U>, Inner: StaticList<T> + ScalarBitOr <U>, U: Clone> ScalarBitOr <U> for Vector<T, Inner> where <Inner as ScalarBitOr <U>>::Output: StaticList<<T as BitOr <U>>::Output> { type Output = Vector<<T as BitOr <U>>::Output, <Inner as ScalarBitOr <U>>::Output>; #[inline] fn scalar_bitor (self, rhs: U) -> Self::Output { Vector(self.0.scalar_bitor (rhs.clone()), self.1.bitor (rhs)) } }
impl<T: BitXor<U>, Inner: StaticList<T> + ScalarBitXor<U>, U: Clone> ScalarBitXor<U> for Vector<T, Inner> where <Inner as ScalarBitXor<U>>::Output: StaticList<<T as BitXor<U>>::Output> { type Output = Vector<<T as BitXor<U>>::Output, <Inner as ScalarBitXor<U>>::Output>; #[inline] fn scalar_bitxor(self, rhs: U) -> Self::Output { Vector(self.0.scalar_bitxor(rhs.clone()), self.1.bitxor(rhs)) } }
impl<T: Shl   <U>, Inner: StaticList<T> + ScalarShl   <U>, U: Clone> ScalarShl   <U> for Vector<T, Inner> where <Inner as ScalarShl   <U>>::Output: StaticList<<T as Shl   <U>>::Output> { type Output = Vector<<T as Shl   <U>>::Output, <Inner as ScalarShl   <U>>::Output>; #[inline] fn scalar_shl   (self, rhs: U) -> Self::Output { Vector(self.0.scalar_shl   (rhs.clone()), self.1.shl   (rhs)) } }
impl<T: Shr   <U>, Inner: StaticList<T> + ScalarShr   <U>, U: Clone> ScalarShr   <U> for Vector<T, Inner> where <Inner as ScalarShr   <U>>::Output: StaticList<<T as Shr   <U>>::Output> { type Output = Vector<<T as Shr   <U>>::Output, <Inner as ScalarShr   <U>>::Output>; #[inline] fn scalar_shr   (self, rhs: U) -> Self::Output { Vector(self.0.scalar_shr   (rhs.clone()), self.1.shr   (rhs)) } }


// Custom

impl<T, U>
    Dot<Vec1<U>>
for Vec1<T>
where
    T: Mul<U>,
{
    type Output = <T as Mul<U>>::Output;
    #[inline]
    fn dot(self, rhs: Vec1<U>) -> Self::Output {
        self.1 * rhs.1
    }
}

impl<
    T, U,
    Inner: StaticList<T>,
    UInner: StaticList<U>,
>
    Dot<Vector<U, Vector<U, UInner>>>
for Vector<T, Vector<T, Inner>>
where
    T: Mul<U>,
    Vector<T, Inner>: Dot<Vector<U, UInner>>,
    <Vector<T, Inner> as Dot<Vector<U, UInner>>>::Output: Add<<T as Mul<U>>::Output>,
{
    type Output = <<Vector<T, Inner> as Dot<Vector<U, UInner>>>::Output as Add<<T as Mul<U>>::Output>>::Output;
    #[inline]
    fn dot(self, rhs: Vector<U, Vector<U, UInner>>) -> Self::Output {
        self.0.dot(rhs.0) + self.1 * rhs.1
    }
}


impl<T, U>
    Cross<Vec3<U>>
for Vec3<T>
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

