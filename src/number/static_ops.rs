use crate::number::{Add1, Num0, Number};


/// Trait for subtracting [`Number`]s within trait bound contexts.
pub trait StaticMinus<N: Number>: Number {
    type Difference: Number;
}

impl<N: Number> StaticMinus<Num0> for N {
    type Difference = N;
}

impl<
    LeftN: Number,
    RightN: Number,
>
    StaticMinus<Add1<RightN>>
for Add1<LeftN>
where
    LeftN: StaticMinus<RightN>,
{
    type Difference = <LeftN as StaticMinus<RightN>>::Difference;
}

