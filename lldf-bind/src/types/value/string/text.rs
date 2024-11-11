use super::*;
use core::mem::transmute_unchecked;


#[derive(Clone)]
pub struct Text {
    _opaque_type : u8
}

impl<T : DFValue> From<T> for Text {
    #[inline(always)]
    fn from(value : T) -> Text { unsafe {
        DF_ACTION__SetVariable_Text_InheritStyles_False_TextValueMerging_NoSpaces(value.to_opaque())
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

// TODO: Add

impl String {

    // TODO: merge

    // TODO: from_minimsg

    // TODO: to_minimsg

    // TODO: unformat

    // TODO: splice

    // TODO: len

}


unsafe impl DFValue for Text {
    #[inline(always)]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


extern "C" {

    fn DF_TRANSMUTE__Opaque( from : Text ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_Text_InheritStyles_False_TextValueMerging_NoSpaces( from : DFOpaqueValue ) -> Text;

}
