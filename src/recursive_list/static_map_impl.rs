use crate::{recursive_list::{BaseCase, RecursiveCase, RecursiveList}, static_list::{StaticMap, StaticZip}};


trait ByCaseStaticMap<T, U, Case>:
    StaticMap<T, U>
{
    fn by_case_map<F>(self, f: &F) -> Self::MapOutput
    where
        F: Fn(T) -> U;
}

impl<T, U, List> StaticMap<T, U> for List
where
    List: RecursiveList<T> + ByCaseStaticMap<T, U, List::Case>,
{
    type MapOutput = List::Same<U>;
    
    #[inline]
    fn map<F>(self, f: &F) -> Self::MapOutput
    where
        F: Fn(T) -> U,
    {
        self.by_case_map(f)
    }
}


impl<T, U, List> ByCaseStaticMap<T, U, BaseCase> for List
where
    List: RecursiveList<T, Case = BaseCase> + StaticMap<T, U>,
    List::Same<U>: RecursiveList<U, Base = List::MapOutput>,
{
    #[inline]
    fn by_case_map<F>(self, _f: &F) -> List::MapOutput
    where
        F: Fn(T) -> U,
    {
        <List::Same<U> as RecursiveList<U>>::BASE
    }
}

impl<T, U, List> ByCaseStaticMap<T, U, RecursiveCase> for List
where
    List: RecursiveList<T, Case = RecursiveCase>,
    List::Inner: StaticMap<T, U>,
    <List::Inner as StaticMap<T, U>>::MapOutput: RecursiveList<U>,
    List: StaticMap<T, U, MapOutput = <<List::Inner as StaticMap<T, U>>::MapOutput as RecursiveList<U>>::Push>,
{
    #[inline]
    fn by_case_map<F>(self, f: &F) -> List::MapOutput
    where
        F: Fn(T) -> U,
    {
        let contents = self.contents();
        contents.inner.map(f).push(f(contents.end))
    }
}



trait ByCaseStaticZip<T, U, R, ListR, Case>:
    StaticZip<T, U, R, ListR>
{
    fn by_case_zip<F>(self, rhs: ListR, f: &F) -> Self::ZipOutput
    where
        F: Fn(T, R) -> U;
}

impl<T, U, R, ListR, List> StaticZip<T, U, R, ListR> for List
where
    List: RecursiveList<T> + ByCaseStaticZip<T, U, R, ListR, List::Case>,
{
    type ZipOutput = List::Same<U>;
    
    #[inline]
    fn zip<F>(self, rhs: ListR, f: &F) -> Self::ZipOutput
    where
        F: Fn(T, R) -> U,
    {
        self.by_case_zip(rhs, f)
    }
}


impl<T, U, R, ListR, List> ByCaseStaticZip<T, U, R, ListR, BaseCase> for List
where
    List: RecursiveList<T, Case = BaseCase> + StaticZip<T, U, R, ListR>,
    List::Same<U>: RecursiveList<U, Base = List::ZipOutput>,
{
    #[inline]
    fn by_case_zip<F>(self, _rhs: ListR, _f: &F) -> List::ZipOutput
    where
        F: Fn(T, R) -> U,
    {
        <List::Same<U> as RecursiveList<U>>::BASE
    }
}

impl<T, U, R, ListR, List> ByCaseStaticZip<T, U, R, ListR, RecursiveCase> for List
where
    List: RecursiveList<T, Case = RecursiveCase> + StaticZip<T, U, R, ListR>,
    ListR: RecursiveList<R, Case = RecursiveCase>,
    List::Inner: StaticZip<T, U, R, ListR::Inner>,
    <List::Inner as StaticZip<T, U, R, ListR::Inner>>::ZipOutput: RecursiveList<U>,
    List: StaticZip<T, U, R, ListR, ZipOutput = <<List::Inner as StaticZip<T, U, R, ListR::Inner>>::ZipOutput as RecursiveList<U>>::Push>,
{
    
    #[inline]
    fn by_case_zip<F>(self, rhs: ListR, f: &F) -> List::ZipOutput
    where
        F: Fn(T, R) -> U,
    {
        let contents = self.contents();
        let rhs_contents = rhs.contents();
        contents.inner.zip(rhs_contents.inner, f).push(f(contents.end, rhs_contents.end))
    }
}

