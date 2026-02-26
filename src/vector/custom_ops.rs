use std::ops::{Add, Mul, Sub};

use crate::{Vec1, Vec3, Vector, VectorTrait, vec3};



pub trait Dot<Rhs = Self> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}
impl<LeftT, RightT> Dot<Vec1<RightT>> for Vec1<LeftT>
where LeftT: Mul<RightT>
{
    type Output = <LeftT as Mul<RightT>>::Output;
    fn dot(self, rhs: Vec1<RightT>) -> Self::Output {
        self.1 * rhs.1
    }
}
impl<LeftT, LeftInner, RightT, RightInner> Dot<Vector<RightT, Vector<RightT, RightInner>>> for Vector<LeftT, Vector<LeftT, LeftInner>>
where
    LeftT: Mul<RightT>,
    LeftInner: VectorTrait<LeftT>,
    RightInner: VectorTrait<RightT>,
    Vector<LeftT, LeftInner>: Dot<Vector<RightT, RightInner>>,
    <Vector<LeftT, LeftInner> as Dot<Vector<RightT, RightInner>>>::Output: Add<<LeftT as Mul<RightT>>::Output>,
{
    type Output = <<Vector<LeftT, LeftInner> as Dot<Vector<RightT, RightInner>>>::Output as Add<<LeftT as Mul<RightT>>::Output>>::Output;
    fn dot(self, rhs: Vector<RightT, Vector<RightT, RightInner>>) -> Self::Output {
        self.0.dot(rhs.0) + self.1 * rhs.1
    }
}


impl<LeftT> Vec3<LeftT> {
    pub fn cross<RightT>(self, rhs: Vec3<RightT>) -> Vec3<<<LeftT as Mul<RightT>>::Output as Sub<<LeftT as Mul<RightT>>::Output>>::Output>
    where
        LeftT: Copy + Mul<RightT>,
        RightT: Copy,
        <LeftT as Mul<RightT>>::Output: Sub<<LeftT as Mul<RightT>>::Output>,
        // <<LeftT as Mul<RightT>>::Output as Add<<LeftT as Mul<RightT>>::Output>>::Output: Add<<LeftT as Mul<RightT>>::Output>,
    {
        vec3(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

