use crate::core::ops::{ Index, IndexMut };


impl<T> Index<usize> for [T] {
    type Output = T;
    #[inline(always)]
    fn index(&self, index : usize) -> &Self::Output {
        loop {}
    }
}

impl<T> IndexMut<usize> for [T] {
    #[inline(always)]
    fn index_mut(&mut self, index : usize) -> &mut Self::Output {
        loop {}
    }
}
