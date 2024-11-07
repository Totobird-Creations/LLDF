use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use core::mem::transmute_unchecked;


#[derive(Clone)]
pub struct Text {
    _opaque_type : u8
}

impl<T : DFValue> From<T> for Text {
    #[inline(always)]
    fn from(value : T) -> Text { unsafe {
        DF_ACTION__SetVariable_Text(value.to_opaque())
    } }
}

impl From<&str> for Text {
    #[inline(always)]
    fn from(value : &str) -> Text {
        Self::from(String::from(value))
    }
}
impl AsRef<Text> for &str {
    #[inline(always)]
    fn as_ref(&self) -> &Text { unsafe {
        transmute_unchecked(&Text::from(*self))
    } }
}


unsafe impl DFValue for Text {
    #[inline(always)]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__Opaque( from : Text ) -> DFOpaqueValue;

    // TODO: Add spaces tags.
    fn DF_ACTION__SetVariable_Text( from : DFOpaqueValue ) -> Text;

}
