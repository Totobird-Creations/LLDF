use crate::prelude::*;
use crate::core::marker::PhantomData;
use crate::bind::DFOpaqueValue;
use core::mem::transmute_unchecked;


#[repr(transparent)]
pub struct List<T : DFValue> {
    _opaque_type : u8,
    _ph          : PhantomData<T>
}

impl<T : DFValue> List<T> {
    #[inline(always)]
    pub fn from_array<const N : usize>(array : [T; N]) -> Self { unsafe {
        transmute_unchecked(DF_TRANSMUTE__Array_Opaque(array))
    } }
}

impl<T : DFValue> Clone for List<T> {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe {
        transmute_unchecked(self._opaque_type.clone())
    } }
}

impl List<String> {

    // TODO: join

}

unsafe impl<T : DFValue> DFValue for List<T> {
    #[inline(always)]
    unsafe fn to_opaque(self) -> DFOpaqueValue {
        DF_TRANSMUTE__ListOpaque_Opaque(transmute_unchecked(self))
    }
}





extern "C" {

    fn DF_TRANSMUTE__ListOpaque_Opaque( from : List<DFOpaqueValue> ) -> DFOpaqueValue;

    fn DF_TRANSMUTE__Array_Opaque( ... ) -> DFOpaqueValue;

}
