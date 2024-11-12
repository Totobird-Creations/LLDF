use crate::prelude::*;
use crate::core::marker::PhantomData;
use crate::bind::DFOpaqueValue;
use core::mem::transmute_unchecked;


#[repr(transparent)]
pub struct List<T : DFValue> {
    _opaque_type : u8,
    _ph          : PhantomData<T>
}

impl<T : DFValue> Clone for List<T> {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_Specialcharequals(transmute_unchecked(self._opaque_type.clone())))
    } }
}

impl<T : DFValue> List<T> {

    #[lldf_bind_proc::dfdoc(SetVariable/CreateList)]
    #[inline(always)]
    pub fn new() -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_CreateList())
    } }

    #[inline(always)]
    pub fn from_array<const N : usize>(array : [T; N]) -> Self { unsafe { // FIXME when arrays are made GEP-compatible
        transmute_unchecked(DF_TRANSMUTE__Array_Opaque(array))
    } }

}

impl<T : DFValue> List<T> {

    #[lldf_bind_proc::dfdoc(SetVariable/AppendValue)]
    #[inline(always)]
    pub fn push(&mut self, value : T) -> () { unsafe {
        DF_ACTION__SetVariable_AppendValue(self.to_opaque(), value.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/AppendList)]
    #[inline(always)]
    pub fn append(&mut self, list : List<T>) -> () { unsafe {
        DF_ACTION__SetVariable_AppendList(self.to_opaque(), list.to_opaque())
    } }

    // TODO: get

    // TODO: pop_unchecked

    // TODO: pop (with bounds check)

    // TODO: remove_unchecked

    // TODO: remove (with bounds check)

    // TODO: set_unchecked

    // TODO: set (with bounds check)

    // TODO: index_of

    // TODO: len

    // TODO: insert_unchecked

    // TODO: insert (with bounds check)

    // TODO: erase (remove by equality)

    // TODO: clear

    // TODO: dedup

    // TODO: splice

    // TODO: sorted

    // TODO: reversed

    // TODO: shuffled

    // TODO: flattened

    // TODO: random (random element)

}

impl List<String> {

    // TODO: join

}

unsafe impl<T : DFValue> DFValue for List<T> {
    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue {
        transmute_unchecked(self._opaque_type.clone())
    }
}





extern "C" {

    fn DF_TRANSMUTE__Array_Opaque( ... ) -> DFOpaqueValue;


    fn DF_ACTION__SetVariable_Specialcharequals( from : DFOpaqueValue ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_CreateList( ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_AppendValue( list : DFOpaqueValue, value : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_AppendList( list : DFOpaqueValue, from : DFOpaqueValue ) -> ();

}
