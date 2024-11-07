use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


#[derive(Clone)]
pub struct String {
    _opaque_type : u8
}

impl<T : DFValue> From<T> for String {
    #[inline(always)]
    fn from(value : T) -> String { unsafe {
        DF_ACTION__SetVariable_String(value.to_opaque())
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

    // TODO: Add spaces tag.
    fn DF_ACTION__SetVariable_String( from : DFOpaqueValue ) -> String;

}
