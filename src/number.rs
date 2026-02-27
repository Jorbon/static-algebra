
trait Sealed {}
#[allow(private_bounds)]
pub trait Number: Sealed {
    const VALUE: usize;
}

pub struct Number0;
pub struct Add1<N: Number>(core::marker::PhantomData<N>);

impl Sealed for Number0 {}
impl<N: Number> Sealed for Add1<N> {}

impl Number for Number0 {
    const VALUE: usize = 0;
}
impl<N: Number> Number for Add1<N> {
    const VALUE: usize = N::VALUE + 1;
}

pub trait StaticMinus<N: Number>: Number {
    type Difference: Number;
}

impl<N: Number> StaticMinus<Number0> for N {
    type Difference = N;
}

impl<LeftN: Number, RightN: Number> StaticMinus<Add1<RightN>> for Add1<LeftN>
where LeftN: StaticMinus<RightN>
{
    type Difference = <LeftN as StaticMinus<RightN>>::Difference;
}

pub type Number1 = Add1<Number0>;
pub type Number2 = Add1<Number1>;
pub type Number3 = Add1<Number2>;
pub type Number4 = Add1<Number3>;
pub type Number5 = Add1<Number4>;
pub type Number6 = Add1<Number5>;
pub type Number7 = Add1<Number6>;
pub type Number8 = Add1<Number7>;
pub type Number9 = Add1<Number8>;

