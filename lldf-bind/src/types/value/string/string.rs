use super::*;
use crate::core::mem::transmute_unchecked;


//#[derive(Clone)]
pub struct String {
    _opaque_type : u8
}

impl Clone for String {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe {
        transmute_unchecked(self._opaque_type.clone())
    } }
}

impl<T : DFValue> From<T> for String {
    #[inline(always)]
    fn from(value : T) -> String { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces(value.to_opaque())
    } }
}

impl From<&str> for String {
    #[inline(always)]
    fn from(value : &str) -> String { unsafe {
        transmute_unchecked::<_, &String>(&value).clone()
    } }
}

unsafe impl DFValue for String {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


extern "C" {

    fn DF_TRANSMUTE__Opaque( from : String ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces( from : DFOpaqueValue ) -> String;

}
