use super::*;
use crate::core::ops::Add;
use core::mem::transmute_unchecked;


/// Text with extra formatting via MiniMessage tags. Used for chat messages, item names, etc.
pub struct Text {
    _opaque_type : u64
}

impl Clone for Text {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe { transmute_unchecked(self._opaque_type) } }
}

impl Text {
    #[inline(always)]
    pub fn new() -> Self { Text::from("") }
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

impl<T : Into<Text>> Add<T> for Text {
    type Output = Text;
    #[inline(always)]
    fn add(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces(self, rhs.into())
    } }
}

impl Text {

    #[lldf_bind_proc::dfdoc(SetVariable/ParseMiniMessageExpr)]
    #[inline(always)]
    pub fn from_minimsg<S : Into<String>>(minimsg : S) -> Text { unsafe {
        DF_ACTION__SetVariable_ParseMiniMessageExpr_ParseLegacyColorCodes_False_AllowedTags_Full(minimsg.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetMiniMessageExpr)]
    #[inline(always)]
    pub fn to_minimsg(&self) -> String { unsafe {
        DF_ACTION__SetVariable_GetMiniMessageExpr(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ClearFormatting)]
    #[inline(always)]
    pub fn without_fmt(&self) -> String { unsafe {
        DF_ACTION__SetVariable_ClearFormatting(self.to_opaque())
    } }

    // TODO: splice

    #[lldf_bind_proc::dfdoc(SetVariable/ContentLength)]
    #[inline(always)]
    pub fn len(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_ContentLength(self.to_opaque())
    } }

}


unsafe impl DFValue for Text {

    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe { transmute_unchecked(self._opaque_type) } }

    #[inline(always)]
    fn to_string(&self) -> String { self.without_fmt() }

    #[inline(always)]
    fn to_text(&self) -> Text { self.clone() }

}


extern "C" {

    fn DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces( ... ) -> Text;

    fn DF_ACTION__SetVariable_ParseMiniMessageExpr_ParseLegacyColorCodes_False_AllowedTags_Full( minimsg : String ) -> Text;
    fn DF_ACTION__SetVariable_GetMiniMessageExpr( from : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_ClearFormatting( text : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_ContentLength( text : DFOpaqueValue ) -> UInt;

}
