use crate::prelude::*;
use crate::core::ops::{ Index, IndexMut };
use crate::core::marker::PhantomData;
use crate::core::mem::MaybeUninit;
use crate::bind::DFOpaqueValue;


pub struct List<T : DFValue> {
    _opaque_type : u8,
    _ph          : PhantomData<T>
}

impl<T : DFValue> Clone for List<T> {
    fn clone(&self) -> Self { unsafe {
        let mut out : MaybeUninit<List<T>> = MaybeUninit::uninit();
        DF_ACTION__SetVariable_SpecialcharEquals((&mut out) as *mut _ as *mut DFOpaqueValue, self as *const _ as *const DFOpaqueValue);
        out.assume_init()
    } }
}

// TODO: Index by Into<Int> (signed).
impl<T : DFValue, U : Into<UInt>> Index<U> for List<T> {
    type Output = T;
    fn index<'l>(&'l self, index : U) -> &'l Self::Output { unsafe {
        // TODO: Add bounds check.
        &*(DF_ACTIONPTR__SetVariable_GetListValue__SetVariable_SetListValue(self as *const _ as *mut DFOpaqueValue, index.into() + 1usize) as *const T)
    } }
}
impl<T : DFValue, U : Into<UInt>> IndexMut<U> for List<T> {
    fn index_mut<'l>(&'l mut self, index : U) -> &'l mut <Self as Index<U>>::Output { unsafe {
        // TODO: Add bounds check.
        &mut *(DF_ACTIONPTR__SetVariable_GetListValue__SetVariable_SetListValue(self as *mut _ as *mut DFOpaqueValue, index.into() + 1usize) as *mut T)
    } }
}

unsafe impl<T : DFValue> DFValue for List<T> { }





extern "C" {

    fn DF_ACTION__SetVariable_SpecialcharEquals( dest : *mut DFOpaqueValue, value : *const DFOpaqueValue ) -> ();

    fn DF_ACTIONPTR__SetVariable_GetListValue__SetVariable_SetListValue( list : *mut DFOpaqueValue, index : UInt ) -> *mut DFOpaqueValue;

}
