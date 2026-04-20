use crate::{recursive_list::{BaseCase, RecursiveCase, RecursiveList}, static_list::Iterable};


trait ByCaseIterable<T, Case> {
    fn by_case_iter(self) -> impl Iterator<Item = T>;
}

impl<T, List> Iterable<T> for List
where
    List: RecursiveList<T> + ByCaseIterable<T, List::Case>,
{
    #[inline]
    fn iter(self) -> impl Iterator<Item = T> {
        self.by_case_iter()
    }
}


impl<T, List> ByCaseIterable<T, BaseCase> for List
where
    List: RecursiveList<T, Case = BaseCase>,
{
    #[inline]
    fn by_case_iter(self) -> impl Iterator<Item = T> {
        core::iter::empty()
    }
}

impl<T, List> ByCaseIterable<T, RecursiveCase> for List
where
    List: RecursiveList<T, Case = RecursiveCase>,
    List::Inner: Iterable<T>,
{
    #[inline]
    fn by_case_iter(self) -> impl Iterator<Item = T> {
        let contents = self.contents();
        contents.inner.iter().chain(core::iter::once(contents.end))
    }
}

