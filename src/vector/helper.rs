use crate::{number::{Num0, Num1, Num2, Num3, Number}, recursive_list::RecursiveList, static_list::StaticGet, vector::{Vec0, VecPush}};


impl<T, Inner> VecPush<T, Inner>
where
    Inner: RecursiveList<T>,
{
    // #[inline]
    // pub fn dot<Rhs>(self, rhs: Rhs) -> <Self as crate::ops::Dot<Rhs>>::Output
    // where
    //     Self: crate::ops::Dot<Rhs>,
    // {
    //     <Self as crate::ops::Dot<Rhs>>::dot(self, rhs)
    // }
    
    #[inline]
    pub fn get<Index>(self) -> T
    where
        Index: Number,
        Self: StaticGet<T, Index>,
    {
        self.static_get()
    }
    
    #[inline]
    pub fn get_ref<'a, Index>(&'a self) -> &'a T
    where
        Index: Number,
        &'a Self: StaticGet<&'a T, Index>,
    {
        self.static_get()
    }
    
    #[inline]
    pub fn get_mut<'a, Index>(&'a mut self) -> &'a mut T
    where
        Index: Number,
        &'a mut Self: StaticGet<&'a mut T, Index>,
    {
        self.static_get()
    }
    
    #[inline]
    pub fn x(self) -> T
    where
        Self: StaticGet<T, Num0>,
    {
        self.get::<Num0>()
    }
    
    #[inline]
    pub fn y(self) -> T
    where
        Self: StaticGet<T, Num1>,
    {
        self.get::<Num1>()
    }
    
    #[inline]
    pub fn z(self) -> T
    where
        Self: StaticGet<T, Num2>,
    {
        self.get::<Num2>()
    }
    
    #[inline]
    pub fn w(self) -> T
    where
        Self: StaticGet<T, Num3>,
    {
        self.get::<Num3>()
    }
}


pub type Vec1<T> = VecPush<T, Vec0>;
pub type Vec2<T> = VecPush<T, Vec1<T>>;
pub type Vec3<T> = VecPush<T, Vec2<T>>;
pub type Vec4<T> = VecPush<T, Vec3<T>>;

impl<T> Vec1<T> {
    #[inline]
    pub const fn new(x: T) -> Self {
        VecPush { inner: Vec0, end: x }
    }
}

impl<T> Vec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        VecPush { inner: VecPush { inner: Vec0, end: x }, end: y }
    }
}

impl<T> Vec3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        VecPush { inner: VecPush { inner: VecPush { inner: Vec0, end: x }, end: y }, end: z }
    }
}

impl<T> Vec4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        VecPush { inner: VecPush { inner: VecPush { inner: VecPush { inner: Vec0, end: x }, end: y }, end: z }, end: w }
    }
}


#[inline]
pub const fn vec1<T>(x: T) -> Vec1<T> {
    Vec1::new(x)
}

#[inline]
pub const fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2::new(x, y)
}

#[inline]
pub const fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3::new(x, y, z)
}

#[inline]
pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4::new(x, y, z, w)
}

