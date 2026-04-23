use core::ops::*;

use crate::{recursive_list::RecursiveCase, vector::Vector};


pub trait DotProduct  <Rhs = Self> { type Output; fn dot_product  (self, rhs: Rhs) -> Self::Output; }
pub trait CrossProduct<Rhs = Self> { type Output; fn cross_product(self, rhs: Rhs) -> Self::Output; }
pub trait InnerProduct<Rhs = Self> { type Output; fn inner_product(self, rhs: Rhs) -> Self::Output; }
pub trait OuterProduct<Rhs = Self> { type Output; fn outer_product(self, rhs: Rhs) -> Self::Output; }


impl<T, Vec, R, VecR> DotProduct<VecR> for Vec
where
    Vec: 

impl<T, Vec, R, VecR> DotProduct<VecR> for Vec
where
    Vec: Vector<T, Case = RecursiveCase>,
    VecR: Vector<R, Case = RecursiveCase>,
    Vec::Inner: Vector<T, Case = RecursiveCase> + DotProduct<VecR::Inner>,
    VecR::Inner: Vector<R, Case = RecursiveCase>,
    T: Mul<R>,
    <T as Mul<R>>::Output: Add<<Vec::Inner as DotProduct<VecR::Inner>>::Output>,
{
    type Output = <<T as Mul<R>>::Output as Add<<Vec::Inner as DotProduct<VecR::Inner>>::Output>>::Output;
    
    #[inline]
    fn dot_product(self, rhs: VecR) -> Self::Output {
        let contents = self.contents();
        let rhs_contents = rhs.contents();
        contents.inner.dot_product(rhs_contents.inner) + contents.end * rhs_contents.end
    }
}

