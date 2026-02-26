use std::marker::PhantomData;

pub trait CountTrait {
    const VALUE: usize;
}

pub struct Count0;
pub struct Count<Inner: CountTrait>(PhantomData<Inner>);

impl CountTrait for Count0 {
    const VALUE: usize = 0;
}
impl<Inner: CountTrait> CountTrait for Count<Inner> {
    const VALUE: usize = Inner::VALUE + 1;
}

pub trait StaticMinus<C: CountTrait>: CountTrait {
    type Difference: CountTrait;
}

impl<C: CountTrait> StaticMinus<Count0> for C {
    type Difference = C;
}

impl<LeftInner, RightInner> StaticMinus<Count<RightInner>> for Count<LeftInner>
where
    LeftInner: CountTrait + StaticMinus<RightInner>,
    RightInner: CountTrait,
{
    type Difference = <LeftInner as StaticMinus<RightInner>>::Difference;
}

pub type Count1 = Count<Count0>;
pub type Count2 = Count<Count1>;
pub type Count3 = Count<Count2>;
pub type Count4 = Count<Count3>;

