use crate::{Number, NumberList, NumberList0, NumberListPush, StaticIndex, StaticList, StaticListBase, StaticListRecursiveOwned, Vector};


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct VectorView<'vector, T, List: StaticList<T>, Indices: NumberList>(
    &'vector List,
    core::marker::PhantomData<(T, Indices)>,
);

impl<'vector, T, Inner: StaticList<T>, Indices: NumberList> StaticList<&'vector T> for VectorView<'vector, T, Inner, Indices> {
    type Length = Indices::Length;
}

impl<'vector, T, List: StaticList<T>> StaticListBase<&'vector T> for VectorView<'vector, T, List, NumberList0> {}

impl<'vector, T, List: StaticList<T>, InnerIndices: NumberList, EndIndex: Number> StaticListRecursiveOwned<&'vector T> for VectorView<'vector, T, List, NumberListPush<InnerIndices, EndIndex>>
where
    List: StaticIndex<T, EndIndex>
{
    type Inner = VectorView<'vector, T, List, InnerIndices>;
    
    #[inline]
    fn inner_owned(self) -> Self::Inner {
        VectorView(self.0, core::marker::PhantomData)
    }
    
    #[inline]
    fn end_owned(self) -> &'vector T {
        <List as StaticIndex<T, EndIndex>>::static_index(self.0)
    }
}


// impl<T, Inner: StaticList<T>> Vector<T, Inner> {
//     fn view<'vector>(&'vector self) -> VectorView<'vector, T, Self, >
// }

