use crate::{recursive_list::{BaseCase, RecursiveCase, RecursiveList}, static_list::{Mappable, StaticMap, StaticZip}};


trait ByCaseStaticMap<T, R, Case>:
    StaticMap<T, R>
{
    fn by_case_map<F>(self, f: &F) -> Self::Mapped
    where
        F: Fn(T) -> R;
}

impl<List, R> StaticMap<List::T, R> for List
where
    List: RecursiveList + ByCaseStaticMap<List::T, R, List::Case>,
{
    #[inline]
    fn map<F>(self, f: &F) -> Self::Mapped
    where
        F: Fn(List::T) -> R,
    {
        self.by_case_map(f)
    }
}


impl<List, R> ByCaseStaticMap<List::T, R, BaseCase> for List
where
    List: RecursiveList<Case = BaseCase> + Mappable<R>,
    List::Mapped: RecursiveList<T = R, Base = List::Mapped>,
{
    #[inline]
    fn by_case_map<F>(self, _f: &F) -> List::Mapped
    where
        F: Fn(List::T) -> R,
    {
        <List::Mapped as RecursiveList>::BASE
    }
}

impl<List, R> ByCaseStaticMap<List::T, R, RecursiveCase> for List
where
    List: RecursiveList<Case = RecursiveCase>,
    List::Inner: StaticMap<List::T, R>,
    <List::Inner as Mappable<R>>::Mapped: RecursiveList<T = R>,
    List: Mappable<R, Mapped = <<List::Inner as Mappable<R>>::Mapped as RecursiveList>::Push>,
{
    #[inline]
    fn by_case_map<F>(self, f: &F) -> List::Mapped
    where
        F: Fn(List::T) -> R,
    {
        let contents = self.contents();
        contents.inner.map(f).push(f(contents.end))
    }
}



trait ByCaseStaticZip<Rhs, T, U, R, Case>:
    StaticZip<Rhs, T, U, R>
{
    fn by_case_zip<F>(self, rhs: Rhs, f: &F) -> Self::Mapped
    where
        F: Fn(T, U) -> R;
}

impl<List, Rhs, R> StaticZip<Rhs, List::T, Rhs::T, R> for List
where
    List: RecursiveList + ByCaseStaticZip<Rhs, List::T, Rhs::T, R, List::Case>,
    Rhs: RecursiveList,
{
    #[inline]
    fn zip<F>(self, rhs: Rhs, f: &F) -> Self::Mapped
    where
        F: Fn(List::T, Rhs::T) -> R,
    {
        self.by_case_zip(rhs, f)
    }
}


impl<List, Rhs, R> ByCaseStaticZip<Rhs, List::T, Rhs::T, R, BaseCase> for List
where
    List: RecursiveList<Case = BaseCase> + Mappable<R>,
    Rhs: RecursiveList<Case = BaseCase>,
    List::Mapped: RecursiveList<T = R, Base = List::Mapped>,
{
    #[inline]
    fn by_case_zip<F>(self, _rhs: Rhs, _f: &F) -> List::Mapped
    where
        F: Fn(List::T, Rhs::T) -> R,
    {
        <List::Mapped as RecursiveList>::BASE
    }
}

impl<List, Rhs, R> ByCaseStaticZip<Rhs, List::T, Rhs::T, R, RecursiveCase> for List
where
    List: RecursiveList<Case = RecursiveCase>,
    Rhs: RecursiveList<Case = RecursiveCase>,
    List::Inner: StaticZip<Rhs::Inner, List::T, Rhs::T, R>,
    <List::Inner as Mappable<R>>::Mapped: RecursiveList<T = R>,
    List: Mappable<R, Mapped = <<List::Inner as Mappable<R>>::Mapped as RecursiveList>::Push>,
{
    #[inline]
    fn by_case_zip<F>(self, rhs: Rhs, f: &F) -> List::Mapped
    where
        F: Fn(List::T, Rhs::T) -> R,
    {
        let contents = self.contents();
        let rhs_contents = rhs.contents();
        contents.inner.zip(rhs_contents.inner, f).push(f(contents.end, rhs_contents.end))
    }
}

