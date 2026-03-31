use crate::{Number, Num0, Num1, Num2, Num3, StaticIndex, StaticIndexMut, StaticIndexOwned, StaticList, Vec0, Vector};


impl<T, Inner: StaticList<T>> Vector<T, Inner> {
    
    #[inline]
    pub fn get<N: Number>(&self) -> &T
    where Self: StaticIndex<T, N>
    {
        StaticIndex::<T, N>::static_index(self)
    }
    
    #[inline]
    pub fn get_mut<N: Number>(&mut self) -> &mut T
    where Self: StaticIndexMut<T, N>
    {
        StaticIndexMut::<T, N>::static_index_mut(self)
    }
    
    #[inline]
    pub fn get_owned<N: Number>(self) -> T
    where Self: StaticIndexOwned<T, N>
    {
        StaticIndexOwned::<T, N>::static_index_owned(self)
    }
    
    #[inline]
    pub fn x(self) -> T
    where Self: StaticIndexOwned<T, Num0>
    {
        self.get_owned::<Num0>()
    }
    
    #[inline]
    pub fn y(self) -> T
    where Self: StaticIndexOwned<T, Num1>
    {
        self.get_owned::<Num1>()
    }
    
    #[inline]
    pub fn z(self) -> T
    where Self: StaticIndexOwned<T, Num2>
    {
        self.get_owned::<Num2>()
    }
    
    #[inline]
    pub fn w(self) -> T
    where Self: StaticIndexOwned<T, Num3>
    {
        self.get_owned::<Num3>()
    }
}


pub type Vec1<T> = Vector<T, Vec0<T>>;
pub type Vec2<T> = Vector<T, Vec1<T>>;
pub type Vec3<T> = Vector<T, Vec2<T>>;
pub type Vec4<T> = Vector<T, Vec3<T>>;

impl<T> Vec1<T> {
    #[inline]
    pub const fn new(x: T) -> Self {
        Self(Vec0::VALUE, x)
    }
}

impl<T> Vec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self(Vector(Vec0::VALUE, x), y)
    }
}

impl<T> Vec3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self(Vector(Vector(Vec0::VALUE, x), y), z)
    }
}

impl<T> Vec4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self(Vector(Vector(Vector(Vec0::VALUE, x), y), z), w)
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

