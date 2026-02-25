use std::marker::PhantomData;
use std::ops::*;
use num_traits::*;

use crate::count::*;



pub trait ConstIterator<T> {
    type Count: CountTrait;
    const LENGTH: usize = Self::Count::VALUE;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a;
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a;
    fn iter_owned(self) -> impl Iterator<Item = T>;
}

pub trait ConstIndexFromEnd<T, C: CountTrait>: ConstIterator<T> {
    fn index_from_end(&self) -> &T;
    fn index_from_end_mut(&mut self) -> &mut T;
    fn index_from_end_owned(self) -> T;
}

pub trait ConstIndex<T, C>:
    Sized + 
    ConstIterator<T, Count = Count<Self::LengthMinusOne>> + 
    ConstIndexFromEnd<T, <Self::LengthMinusOne as ConstMinus<C>>::Difference>
where C: CountTrait
{
    type LengthMinusOne: CountTrait + ConstMinus<C>;
    fn index(&self) -> &T {
        <Self as ConstIndexFromEnd<T, <Self::LengthMinusOne as ConstMinus<C>>::Difference>>::index_from_end(self)
    }
    fn index_mut(&mut self) -> &mut T {
        <Self as ConstIndexFromEnd<T, <Self::LengthMinusOne as ConstMinus<C>>::Difference>>::index_from_end_mut(self)
    }
    fn index_owned(self) -> T {
        <Self as ConstIndexFromEnd<T, <Self::LengthMinusOne as ConstMinus<C>>::Difference>>::index_from_end_owned(self)
    }
}



// Core

pub trait VectorTrait<T>: ConstIterator<T> {}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Vec0<T>(PhantomData<T>);
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Vector<T, Inner: VectorTrait<T>>(pub Inner, pub T);

impl<T> ConstIterator<T> for Vec0<T> {
    type Count = Count0;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        std::iter::empty()
    }
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        std::iter::empty()
    }
    fn iter_owned(self) -> impl Iterator<Item = T> {
        std::iter::empty()
    }
}
impl<T, Inner: VectorTrait<T>> ConstIterator<T> for Vector<T, Inner> {
    type Count = Count<Inner::Count>;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        self.0.iter().chain(std::iter::once(&self.1))
    }
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        self.0.iter_mut().chain(std::iter::once(&mut self.1))
    }
    fn iter_owned(self) -> impl Iterator<Item = T> {
        self.0.iter_owned().chain(std::iter::once(self.1))
    }
}

impl<T> VectorTrait<T> for Vec0<T> {}
impl<T, Inner: VectorTrait<T>> VectorTrait<T> for Vector<T, Inner> {}


impl<T, Inner: VectorTrait<T>> ConstIndexFromEnd<T, Count0> for Vector<T, Inner> {
    fn index_from_end(&self) -> &T {
        &self.1
    }
    fn index_from_end_mut(&mut self) -> &mut T {
        &mut self.1
    }
    fn index_from_end_owned(self) -> T {
        self.1
    }
}
impl<T, VectorInner, CountInner> ConstIndexFromEnd<T, Count<CountInner>> for Vector<T, VectorInner>
where
    VectorInner: VectorTrait<T> + ConstIndexFromEnd<T, CountInner>,
    CountInner: CountTrait,
{
    fn index_from_end(&self) -> &T {
        <VectorInner as ConstIndexFromEnd<T, CountInner>>::index_from_end(&self.0)
    }
    fn index_from_end_mut(&mut self) -> &mut T {
        <VectorInner as ConstIndexFromEnd<T, CountInner>>::index_from_end_mut(&mut self.0)
    }
    fn index_from_end_owned(self) -> T {
        <VectorInner as ConstIndexFromEnd<T, CountInner>>::index_from_end_owned(self.0)
    }
}

impl<T, Inner, C> ConstIndex<T, C> for Vector<T, Inner>
where
    Inner: VectorTrait<T>,
    C: CountTrait,
    Inner::Count: ConstMinus<C>,
    Self: ConstIterator<T, Count = Count<Inner::Count>> + ConstIndexFromEnd<T, <Inner::Count as ConstMinus<C>>::Difference>,
{
    type LengthMinusOne = Inner::Count;
}

impl<T, Inner: VectorTrait<T>> Vector<T, Inner> {
    pub fn get_ref<C>(&self) -> &T
    where
        C: CountTrait,
        Self: ConstIndex<T, C>,
    {
        ConstIndex::<T, C>::index(self)
    }
    
    pub fn get_mut<C>(&mut self) -> &mut T
    where
        C: CountTrait,
        Self: ConstIndex<T, C>,
    {
        ConstIndex::<T, C>::index_mut(self)
    }
    
    pub fn get<C>(self) -> T
    where
        C: CountTrait,
        Self: ConstIndex<T, C>,
    {
        ConstIndex::<T, C>::index_owned(self)
    }
    
    pub fn x(self) -> T
    where Self: ConstIndex<T, Count0>
    {
        self.get::<Count0>()
    }
    
    pub fn y(self) -> T
    where Self: ConstIndex<T, Count1>
    {
        self.get::<Count1>()
    }
    
    pub fn z(self) -> T
    where Self: ConstIndex<T, Count2>
    {
        self.get::<Count2>()
    }
    
    pub fn w(self) -> T
    where Self: ConstIndex<T, Count3>
    {
        self.get::<Count3>()
    }
}


// Length-specific

pub type Vec1<T> = Vector<T, Vec0<T>>;
pub type Vec2<T> = Vector<T, Vec1<T>>;
pub type Vec3<T> = Vector<T, Vec2<T>>;
pub type Vec4<T> = Vector<T, Vec3<T>>;

pub const fn vec1<T>(x: T) -> Vec1<T> {
    Vector(Vec0(PhantomData), x)
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



// Print
impl<T, Inner> std::fmt::Debug for Vector<T, Inner>
where
    Inner: VectorTrait<T>,
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec{}( ", Self::LENGTH)?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{:?}", value)?;
        }
        write!(f, " )")?;
        Ok(())
    }
}

impl<T, Inner> std::fmt::Display for Vector<T, Inner>
where
    Inner: VectorTrait<T>,
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec{}( ", Self::LENGTH)?;
        for (i, value) in self.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", value)?;
        }
        write!(f, " )")?;
        Ok(())
    }
}


// Props

impl<T> Zero for Vec0<T>
where T: Zero
{
    fn zero() -> Self {
        Self(PhantomData)
    }
    fn is_zero(&self) -> bool {
        true
    }
}
impl<T, Inner> Zero for Vector<T, Inner>
where
    Inner: VectorTrait<T> + Zero,
    T: Zero,
{
    fn zero() -> Self {
        Self(Inner::zero(), T::zero())
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero() && self.1.is_zero()
    }
}

impl<T> ConstZero for Vec0<T>
where T: ConstZero
{
    const ZERO: Self = Self(PhantomData);
}
impl<T, Inner> ConstZero for Vector<T, Inner>
where
    Inner: VectorTrait<T> + ConstZero,
    T: ConstZero,
{
    const ZERO: Self = Vector(Inner::ZERO, T::ZERO);
}


impl<T> From<()> for Vec0<T> {
    #[allow(unused_variables)]
    fn from(value: ()) -> Self {
        Self(PhantomData)
    }
}
impl<T, Inner, IntoT, IntoInner> From<(IntoInner, IntoT)> for Vector<T, Inner>
where
    Inner: VectorTrait<T>,
    IntoT: Into<T>,
    IntoInner: Into<Inner>,
{
    fn from(value: (IntoInner, IntoT)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}


// Ops

impl<T> Neg for Vec0<T>
where T: Neg
{
    type Output = Vec0<<T as Neg>::Output>;
    fn neg(self) -> Self::Output {
        Vec0(PhantomData)
    }
}
impl<T, Inner> Neg for Vector<T, Inner>
where
    Inner: VectorTrait<T> + Neg,
    T: Neg,
    <Inner as Neg>::Output: VectorTrait<<T as Neg>::Output>,
{
    type Output = Vector<<T as Neg>::Output, <Inner as Neg>::Output>;
    fn neg(self) -> Self::Output {
        Vector(-self.0, -self.1)
    }
}

impl<T> Not for Vec0<T>
where T: Not
{
    type Output = Vec0<<T as Not>::Output>;
    fn not(self) -> Self::Output {
        Vec0(PhantomData)
    }
}
impl<T, Inner> Not for Vector<T, Inner>
where
    T: Not,
    Inner: VectorTrait<T> + Not,
    <Inner as Not>::Output: VectorTrait<<T as Not>::Output>,
{
    type Output = Vector<<T as Not>::Output, <Inner as Not>::Output>;
    fn not(self) -> Self::Output {
        Vector(!self.0, !self.1)
    }
}


impl<LeftT, RightT> Add<Vec0<RightT>> for Vec0<LeftT>
where LeftT: Add<RightT>
{
    type Output = Vec0<<LeftT as Add<RightT>>::Output>;
    #[allow(unused_variables)]
    fn add(self, rhs: Vec0<RightT>) -> Self::Output {
        Vec0(PhantomData)
    }
}
impl<LeftT, LeftInner, RightT, RightInner> Add<Vector<RightT, RightInner>> for Vector<LeftT, LeftInner>
where
    LeftT: Add<RightT>,
    LeftInner: VectorTrait<LeftT> + Add<RightInner>,
    RightInner: VectorTrait<RightT>,
    <LeftInner as Add<RightInner>>::Output: VectorTrait<<LeftT as Add<RightT>>::Output>,
{
    type Output = Vector<<LeftT as Add<RightT>>::Output, <LeftInner as Add<RightInner>>::Output>;
    fn add(self, rhs: Vector<RightT, RightInner>) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}


impl<LeftT, RightT> Mul<RightT> for Vec0<LeftT>
where LeftT: Mul<RightT>
{
    type Output = Vec0<<LeftT as Mul<RightT>>::Output>;
    #[allow(unused_variables)]
    fn mul(self, rhs: RightT) -> Self::Output {
        Vec0(PhantomData)
    }
}
impl<LeftT, LeftInner, RightT> Mul<RightT> for Vector<LeftT, LeftInner>
where
    LeftT: Mul<RightT>,
    RightT: Copy,
    LeftInner: VectorTrait<LeftT> + Mul<RightT>,
    <LeftInner as Mul<RightT>>::Output: VectorTrait<<LeftT as Mul<RightT>>::Output>,
{
    type Output = Vector<<LeftT as Mul<RightT>>::Output, <LeftInner as Mul<RightT>>::Output>;
    fn mul(self, rhs: RightT) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs)
    }
}


// Custom ops

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

