use super::*;


pub struct Int {
    pub(crate) _opaque_type : i64
}
impl Clone for Int { fn clone(&self) -> Self { *self } }
impl Copy for Int {}

impl From<i8    > for Int { #[inline(always)] fn from(value : i8    ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<i16   > for Int { #[inline(always)] fn from(value : i16   ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<i32   > for Int { #[inline(always)] fn from(value : i32   ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<i64   > for Int { #[inline(always)] fn from(value : i64   ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<i128  > for Int { #[inline(always)] fn from(value : i128  ) -> Self { unsafe { *(&value as *const _ as *const _) } } }
impl From<isize > for Int { #[inline(always)] fn from(value : isize ) -> Self { unsafe { *(&value as *const _ as *const _) } } }

unsafe impl DFValue for Int {

    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe {
        transmute_unchecked(self._opaque_type)
    } }

    #[inline(always)]
    fn to_string(&self) -> String { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces(self)
    } }

    #[inline(always)]
    fn to_text(&self) -> Text { unsafe {
        DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces(self)
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces( ... ) -> String;
    fn DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces( ... ) -> Text;

}
