use crate::core::ops::{ Index, IndexMut };
use crate::core::mem::transmute_unchecked;


impl<T, I, const N : usize> Index<I> for [T; N] where [T] : Index<I> {
    type Output = <[T] as Index<I>>::Output;
    #[inline(always)]
    fn index(&self, index : I) -> &Self::Output {
        Index::index(unsafe{ transmute_unchecked::<_, &[T]>(self) }, index)
    }
}

impl<T, I, const N : usize> IndexMut<I> for [T; N] where [T] : IndexMut<I> {
    #[inline(always)]
    fn index_mut(&mut self, index : I) -> &mut Self::Output {
        IndexMut::index_mut(unsafe{ transmute_unchecked::<_, &mut [T]>(self) }, index)
    }
}
