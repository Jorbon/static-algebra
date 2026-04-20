
/// Types that have a complex conjugate value.
/// Some operations, such as inner & outer products, require their elements to implement [`Conjugate`].
pub trait Conjugate {
    fn conjugate(self) -> Self;
}

/// Default implementation of [`Conjugate`] for types that have no imaginary component.
pub trait RealValue {}

impl<Value> Conjugate for Value
where
    Value: RealValue,
{
    #[inline]
    fn conjugate(self) -> Self { self }
}


impl RealValue for u8 {}
impl RealValue for u16 {}
impl RealValue for u32 {}
impl RealValue for u64 {}
impl RealValue for u128 {}
impl RealValue for usize {}
impl RealValue for i8 {}
impl RealValue for i16 {}
impl RealValue for i32 {}
impl RealValue for i64 {}
impl RealValue for i128 {}
impl RealValue for isize {}
impl RealValue for f32 {}
impl RealValue for f64 {}


/// Utility type for complex values.
pub struct Complex<T> {
    pub real: T,
    pub imaginary: T,
}

impl<T> Conjugate for Complex<T>
where
    T: core::ops::Neg<Output = T>,
{
    #[inline]
    fn conjugate(self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }
}

