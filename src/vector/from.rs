use crate::vector::{Vec0, helper::{Vec1, Vec2, Vec3, Vec4, vec1, vec2, vec3, vec4}};


impl From<()> for Vec0 {
    #[inline]
    fn from(_value: ()) -> Self {
        Vec0
    }

}

impl<T, IntoT> From<(IntoT,)> for Vec1<T>
where
    IntoT: Into<T>,
{
    #[inline]
    fn from(value: (IntoT,)) -> Self {
        vec1(value.0.into())
    }
}

impl<T, IntoT0, IntoT1> From<(IntoT0, IntoT1)> for Vec2<T>
where
    IntoT0: Into<T>,
    IntoT1: Into<T>,
{
    #[inline]
    fn from(value: (IntoT0, IntoT1)) -> Self {
        vec2(value.0.into(), value.1.into())
    }
}

impl<T, IntoT0, IntoT1, IntoT2> From<(IntoT0, IntoT1, IntoT2)> for Vec3<T>
where
    IntoT0: Into<T>,
    IntoT1: Into<T>,
    IntoT2: Into<T>,
{
    #[inline]
    fn from(value: (IntoT0, IntoT1, IntoT2)) -> Self {
        vec3(value.0.into(), value.1.into(), value.2.into())
    }
}

impl<T, IntoT0, IntoT1, IntoT2, IntoT3> From<(IntoT0, IntoT1, IntoT2, IntoT3)> for Vec4<T>
where
    IntoT0: Into<T>,
    IntoT1: Into<T>,
    IntoT2: Into<T>,
    IntoT3: Into<T>,
{
    #[inline]
    fn from(value: (IntoT0, IntoT1, IntoT2, IntoT3)) -> Self {
        vec4(value.0.into(), value.1.into(), value.2.into(), value.3.into())
    }
}


impl<   IntoT> From<[IntoT; 0]> for Vec0                                 { #[inline] fn from(_value: [IntoT; 0]) -> Self { Vec0 } }
impl<T, IntoT> From<[IntoT; 1]> for Vec1<T> where IntoT: Into<T> + Clone { #[inline] fn from( value: [IntoT; 1]) -> Self { vec1(value[0].clone().into()) } }
impl<T, IntoT> From<[IntoT; 2]> for Vec2<T> where IntoT: Into<T> + Clone { #[inline] fn from( value: [IntoT; 2]) -> Self { vec2(value[0].clone().into(), value[1].clone().into()) } }
impl<T, IntoT> From<[IntoT; 3]> for Vec3<T> where IntoT: Into<T> + Clone { #[inline] fn from( value: [IntoT; 3]) -> Self { vec3(value[0].clone().into(), value[1].clone().into(), value[2].clone().into()) } }
impl<T, IntoT> From<[IntoT; 4]> for Vec4<T> where IntoT: Into<T> + Clone { #[inline] fn from( value: [IntoT; 4]) -> Self { vec4(value[0].clone().into(), value[1].clone().into(), value[2].clone().into(), value[3].clone().into()) } }

