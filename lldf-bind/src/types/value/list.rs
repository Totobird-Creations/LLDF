use crate::prelude::*;
use crate::core::marker::PhantomData;
use crate::core::ops::Index;
use crate::core::mem::{ transmute_unchecked, MaybeUninit };


#[derive(Clone, Copy)]
pub struct List<T : DFValue> {
    _opaque_type : u8,
    _ph          : PhantomData<T>
}

impl<T : DFValue> List<T> {
    pub fn get<U : Into<UInt>>(&self, index : U) -> T { unsafe {
        let mut out : T = MaybeUninit::uninit().assume_init();
        crate::bind::action::DF_ACTION_SetVariable_GetListValue(&mut out, *self, index.into() + 1usize);
        out
    } }
}

impl<T : DFValue, U : Into<UInt>> Index<U> for List<T> {
    type Output = T;
    fn index(&self, index : U) -> &Self::Output { unsafe {
        transmute_unchecked::<_, &_>(&self.get(index))
    } }
}

unsafe impl<T : DFValue> DFValue for List<T> {}
