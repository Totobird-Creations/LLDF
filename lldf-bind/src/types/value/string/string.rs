use super::*;
use crate::core::mem::transmute_unchecked;


/// A sequence of characters.
#[derive(Clone)]
pub struct String {
    _opaque_type : u8
}

//impl Clone for String {
//    #[inline(always)]
//    fn clone(&self) -> Self { unsafe {
//        transmute_unchecked(self._opaque_type.clone())
//    } }
//}

impl String {

    #[lldf_bind_proc::dfdoc(SetVariable/String)]
    #[inline(always)]
    pub fn new() -> Self { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces()
    } }

}

impl<T : DFValue> From<T> for String {
    #[inline(always)]
    fn from(value : T) -> String { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces(value)
    } }
}

impl From<&str> for String {
    #[inline(always)]
    fn from(value : &str) -> String { unsafe { // FIXME: Empty strings (`""`) don't work as expected.
        transmute_unchecked::<_, &String>(&value).clone()
    } }
}

// TODO: Add

impl String {

    // TODO: replace

    // TODO: splice

    // TODO: split

    // TODO: uppercase

    // TODO: lowercase

    // TODO: proper_case

    // TODO: invert_case

    // TODO: random_case

    // TODO: len

    // TODO: repeat

}

unsafe impl DFValue for String {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


extern "C" {

    fn DF_TRANSMUTE__Opaque( from : String ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces( ... ) -> String;

}
