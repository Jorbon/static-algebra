use crate::{StaticIndex, Number0, Number1, Number2, Number3, Number, Vec0, Vector, StaticList, vec0};


impl<T, Inner: StaticList<T>> Vector<T, Inner> {
    pub fn get_ref<N: Number>(&self) -> &T
    where Self: StaticIndex<T, N>
    {
        StaticIndex::<T, N>::static_index(self)
    }
    
    pub fn get_mut<N: Number>(&mut self) -> &mut T
    where Self: StaticIndex<T, N>
    {
        StaticIndex::<T, N>::static_index_mut(self)
    }
    
    pub fn get<N: Number>(self) -> T
    where Self: StaticIndex<T, N>
    {
        StaticIndex::<T, N>::static_index_owned(self)
    }
    
    pub fn x(self) -> T
    where Self: StaticIndex<T, Number0>
    {
        self.get::<Number0>()
    }
    
    pub fn y(self) -> T
    where Self: StaticIndex<T, Number1>
    {
        self.get::<Number1>()
    }
    
    pub fn z(self) -> T
    where Self: StaticIndex<T, Number2>
    {
        self.get::<Number2>()
    }
    
    pub fn w(self) -> T
    where Self: StaticIndex<T, Number3>
    {
        self.get::<Number3>()
    }
}


pub type Vec1<T> = Vector<T, Vec0<T>>;
pub type Vec2<T> = Vector<T, Vec1<T>>;
pub type Vec3<T> = Vector<T, Vec2<T>>;
pub type Vec4<T> = Vector<T, Vec3<T>>;

pub const fn vec1<T>(x: T) -> Vec1<T> {
    Vector(vec0(), x)
}

pub const fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vector(vec1(x), y)
}

pub const fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vector(vec2(x, y), z)
}

pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vector(vec3(x, y, z), w)
}


impl<T> From<[T; 0]> for Vec0<T> {
    fn from(_value: [T; 0]) -> Self {
        vec0()
    }
}

impl<T: Copy> From<[T; 1]> for Vec1<T> {
    fn from(value: [T; 1]) -> Self {
        vec1(value[0])
    }
}

impl<T: Copy> From<[T; 2]> for Vec2<T> {
    fn from(value: [T; 2]) -> Self {
        vec2(value[0], value[1])
    }
}

impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(value: [T; 3]) -> Self {
        vec3(value[0], value[1], value[2])
    }
}

impl<T: Copy> From<[T; 4]> for Vec4<T> {
    fn from(value: [T; 4]) -> Self {
        vec4(value[0], value[1], value[2], value[3])
    }
}

