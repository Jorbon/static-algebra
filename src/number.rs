
trait Sealed {}
#[allow(private_bounds)]
pub trait Number: Sealed {
    const VALUE: usize;
}

pub struct Num0;
pub struct Add1<N: Number>(core::marker::PhantomData<N>);

impl Sealed for Num0 {}
impl<N: Number> Sealed for Add1<N> {}

impl Number for Num0 {
    const VALUE: usize = 0;
}
impl<N: Number> Number for Add1<N> {
    const VALUE: usize = N::VALUE + 1;
}

pub trait StaticMinus<N: Number>: Number {
    type Difference: Number;
}

impl<N: Number> StaticMinus<Num0> for N {
    type Difference = N;
}

impl<LeftN: Number, RightN: Number> StaticMinus<Add1<RightN>> for Add1<LeftN>
where LeftN: StaticMinus<RightN>
{
    type Difference = <LeftN as StaticMinus<RightN>>::Difference;
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

