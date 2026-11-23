use crate::{recursive_list::{BaseCase, RecursiveCase, RecursiveList}, static_list::Iterable};


// Local helper
trait ByCaseIterable<T, Case> {
    fn by_case_iter(self) -> impl Iterator<Item = T>;
}

// Blanket impl on target
impl<List> Iterable for List
where
    List: RecursiveList + ByCaseIterable<List::T, List::Case>,
{
    type Item = List::T;
    
    #[inline]
    fn iter(self) -> impl Iterator<Item = Self::Item> {
        self.by_case_iter()
    }
}

// Local impls

impl<List, T> ByCaseIterable<T, BaseCase> for List
where
    List: RecursiveList<Case = BaseCase>,
{
    #[inline]
    fn by_case_iter(self) -> impl Iterator<Item = T> {
        core::iter::empty()
    }
}

impl<List, T> ByCaseIterable<T, RecursiveCase> for List
where
    List: RecursiveList<Case = RecursiveCase, T = T>,
    List::Inner: Iterable<Item = T>,
{
    #[inline]
    fn by_case_iter(self) -> impl Iterator<Item = T> {
        let contents = self.contents();
        contents.inner.iter().chain(core::iter::once(contents.end))
    }
}

