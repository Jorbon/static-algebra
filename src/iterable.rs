
pub trait Iterable<T> {
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a;
    
    #[inline]
    fn index<'a>(&'a self, index: usize) -> Option<&'a T> where T: 'a {
        self.iter().nth(index)
    }
}

pub trait IterableMut<T> {
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a;
    
    #[inline]
    fn index_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut T> where T: 'a {
        self.iter_mut().nth(index)
    }
}

pub trait IterableOwned<T> {
    fn into_iter(self) -> impl Iterator<Item = T>;
    
    #[inline]
    fn into_index(self, index: usize) -> Option<T> where Self: Sized {
        self.into_iter().nth(index)
    }
}
