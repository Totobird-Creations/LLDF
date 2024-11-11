use crate::core::ops::{ Index, IndexMut };


impl<T> Index<usize> for [T] {
    type Output = T;
    #[inline(always)]
    fn index(&self, _index : usize) -> &Self::Output {
        // TODO: Slice indexing
        loop {}
    }
}

impl<T> IndexMut<usize> for [T] {
    #[inline(always)]
    fn index_mut(&mut self, _index : usize) -> &mut Self::Output {
        // TODO: Slice indexing
        loop {}
    }
}
