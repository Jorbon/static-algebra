use crate::{Number, NumberList, NumberList0, NumberListPush, StaticIndex, StaticList, StaticListBase, StaticListRecursiveOwned};


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct VectorView<'a, T, List: StaticList<T>, Indices: NumberList>(
    &'a List,
    core::marker::PhantomData<(T, Indices)>,
);

impl<'a, T, Inner: StaticList<T>, Indices: NumberList> StaticList<&'a T> for VectorView<'a, T, Inner, Indices> {
    type Length = Indices::Length;
}

impl<'a, T, List: StaticList<T>> StaticListBase<&'a T> for VectorView<'a, T, List, NumberList0> {}

impl<'a, T, List: StaticList<T>, InnerIndices: NumberList, EndIndex: Number> StaticListRecursiveOwned<&'a T> for VectorView<'a, T, List, NumberListPush<InnerIndices, EndIndex>>
where
    List: StaticIndex<T, EndIndex>
{
    type Inner = VectorView<'a, T, List, InnerIndices>;
    
    fn inner_owned(self) -> Self::Inner {
        VectorView(self.0, core::marker::PhantomData)
    }
    
    fn end_owned(self) -> &'a T {
        <List as StaticIndex<T, EndIndex>>::static_index(self.0)
    }
}


