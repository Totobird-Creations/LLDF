use crate::prelude::*;
//use crate::core::ops::{ Index, IndexMut };
use crate::core::marker::PhantomData;
use crate::bind::DFOpaqueValue;
use core::mem::transmute_unchecked;


pub struct List<T : DFValue> {
    _opaque_type : u8,
    _ph          : PhantomData<T>
}

impl<T : DFValue> Clone for List<T> {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe {
        let out = DF_ACTION__SetVariable_Specialcharequals((&*DF_TRANSMUTE__Ptr_Opaque(self)).clone());
        transmute_unchecked(DF_TRANSMUTE__List(out))
    } }
}

// TODO: Index by Into<Int> (signed).
//impl<T : DFValue, U : Into<UInt>> Index<U> for List<T> {
//    type Output = T;
//    #[inline(always)]
//    fn index<'l>(&'l self, index : U) -> &'l Self::Output { unsafe {
//        // TODO: Add bounds check.
//        let index = *((&index.into()) as *const _ as *const usize) + 1;
//        &*(DF_ACTIONPTR__SetVariable_GetListValue__SetVariable_SetListValue(DF_TRANSMUTE__Opaque(self), index) as *const T)
//    } }
//}
//impl<T : DFValue, U : Into<UInt>> IndexMut<U> for List<T> {
//    #[inline(always)]
//    fn index_mut<'l>(&'l mut self, index : U) -> &'l mut <Self as Index<U>>::Output { unsafe {
//        // TODO: Add bounds check.
//        let index = *((&index.into()) as *const _ as *const usize) + 1;
//        &mut *(DF_ACTIONPTR__SetVariable_GetListValue__SetVariable_SetListValue(DF_TRANSMUTE__Opaque(&*self), index) as *mut T)
//    } }
//}

unsafe impl<T : DFValue> DFValue for List<T> {
    #[inline(always)]
    unsafe fn to_opaque(self) -> DFOpaqueValue {
        DF_TRANSMUTE__ListOpaque_Opaque(transmute_unchecked(self))
    }
}





#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__ListOpaque_Opaque( from : List<DFOpaqueValue> ) -> DFOpaqueValue;
    fn DF_TRANSMUTE__Ptr_Opaque( ... ) -> *const DFOpaqueValue;
    fn DF_TRANSMUTE__List( from : DFOpaqueValue ) -> List<DFOpaqueValue>;

    fn DF_ACTION__SetVariable_Specialcharequals( value : DFOpaqueValue ) -> DFOpaqueValue;

    //fn DF_ACTIONPTR__SetVariable_GetListValue__SetVariable_SetListValue( list : DFOpaqueValue, index : usize ) -> *mut DFOpaqueValue;

}
