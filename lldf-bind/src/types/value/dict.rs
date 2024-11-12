use crate::bind::DFOpaqueValue;
use crate::prelude::*;
use crate::core::marker::PhantomData;
use crate::core::mem::transmute_unchecked;


pub struct Dict<T : DFValue> {
    _opaque_type : u8,
    _ph          : PhantomData<T>
}

impl<T : DFValue> Clone for Dict<T> {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_Specialcharequals(transmute_unchecked(self._opaque_type.clone())))
    } }
}

impl<T : DFValue> Dict<T> {

    #[lldf_bind_proc::dfdoc(SetVariable/CreateDict)]
    pub fn new() -> Self { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_CreateDict())
    } }

    // TODO: from_arrays

}

impl<T : DFValue> Dict<T> {

    #[lldf_bind_proc::dfdoc(SetVariable/SetDictValue)]
    #[inline(always)]
    pub fn insert<S : Into<String>>(&mut self, key : S, value : T) -> () { unsafe {
        DF_ACTION__SetVariable_SetDictValue(self.to_opaque(), key.into(), value.to_opaque());
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetDictValue)]
    #[inline(always)]
    pub unsafe fn get_unchecked<S : Into<String>>(&self, key : S) -> T { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_GetDictValue(self.to_opaque(), key.into()))
    } }

    // TODO: get (with contains check)

    #[lldf_bind_proc::dfdoc(SetVariable/GetDictSize)]
    #[inline(always)]
    pub fn size(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_GetDictSize(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/RemoveDictEntry)]
    #[inline(always)]
    pub fn remove<S : Into<String>>(&mut self, key : S) -> () { unsafe {
        DF_ACTION__SetVariable_RemoveDictEntry(self.to_opaque(), key.into());
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ClearDict)]
    #[inline(always)]
    pub fn clear(&mut self) -> () { unsafe {
        DF_ACTION__SetVariable_ClearDict(self.to_opaque());
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetDictKeys)]
    #[inline(always)]
    pub fn keys(&self) -> List<String> { unsafe {
        DF_ACTION__SetVariable_GetDictKeys(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetDictValues)]
    #[inline(always)]
    pub fn values(&self) -> List<T> { unsafe {
        transmute_unchecked(DF_ACTION__SetVariable_GetDictValues(self.to_opaque()))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/AppendDict)]
    #[inline(always)]
    pub fn append(&mut self, other : Self) -> () { unsafe {
        DF_ACTION__SetVariable_AppendDict(self.to_opaque(), other.to_opaque());
    } }

}

unsafe impl<T : DFValue> DFValue for Dict<T> {
    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue {
        transmute_unchecked(self._opaque_type.clone())
    }
}



extern "C" {

    fn DF_ACTION__SetVariable_Specialcharequals( from : DFOpaqueValue ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_CreateDict( ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_SetDictValue( dict : DFOpaqueValue, key : String, value : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_GetDictValue( dict : DFOpaqueValue, key : String ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_GetDictSize( dict : DFOpaqueValue ) -> UInt;
    fn DF_ACTION__SetVariable_RemoveDictEntry( dict : DFOpaqueValue, key : String ) -> ();
    fn DF_ACTION__SetVariable_ClearDict( dict : DFOpaqueValue ) -> ();
    fn DF_ACTION__SetVariable_GetDictKeys( dict : DFOpaqueValue ) -> List<String>;
    fn DF_ACTION__SetVariable_GetDictValues( dict : DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_AppendDict( a : DFOpaqueValue, b : DFOpaqueValue ) -> DFOpaqueValue;

}
