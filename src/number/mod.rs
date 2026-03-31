pub mod static_ops;

pub use static_ops::StaticMinus;


trait NumberSealed {}

/// A non-negative integer represented by a pure type rather than a numeric primitive.
/// The actual numeric value is provided as a `usize` in [`Number::VALUE`].
#[allow(private_bounds)]
pub trait Number: NumberSealed {
    const VALUE: usize;
}

/// [`Number`] of value 0.
pub struct Num0;
/// Recursive type for specifying any [`Number`] value using [`Num0`] as a base case.
/// 
/// Example: [`Add1<Add1<Num0>>`] = [`Num2`] = 2
pub struct Add1<N: Number>(core::marker::PhantomData<N>);

impl NumberSealed for Num0 {}
impl<N: Number> NumberSealed for Add1<N> {}

impl Number for Num0 {
    const VALUE: usize = 0;
}

impl<N: Number> Number for Add1<N> {
    const VALUE: usize = N::VALUE + 1;
}

pub type Num1 = Add1<Num0>;
pub type Num2 = Add1<Num1>;
pub type Num3 = Add1<Num2>;
pub type Num4 = Add1<Num3>;
pub type Num5 = Add1<Num4>;
pub type Num6 = Add1<Num5>;
pub type Num7 = Add1<Num6>;
pub type Num8 = Add1<Num7>;
pub type Num9 = Add1<Num8>;


// pub trait NumberList {
//     type Length: Number;
// }

// pub struct NumberList0;
// pub struct NumberListPush<Inner: NumberList, N: Number>(core::marker::PhantomData<(Inner, N)>);

// impl NumberList for NumberList0 {
//     type Length = Num0;
// }

// impl<Inner: NumberList, N: Number> NumberList for NumberListPush<Inner, N> {
//     type Length = Add1<Inner::Length>;
// }

// pub type NumberList1<N0            > = NumberListPush<NumberList0            , N0>;
// pub type NumberList2<N0, N1        > = NumberListPush<NumberList1<N0>        , N1>;
// pub type NumberList3<N0, N1, N2    > = NumberListPush<NumberList2<N0, N1>    , N2>;
// pub type NumberList4<N0, N1, N2, N3> = NumberListPush<NumberList3<N0, N1, N2>, N3>;

