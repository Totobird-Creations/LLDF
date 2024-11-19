use super::*;
use crate::core::ops::Add;
use crate::core::mem::transmute_unchecked;


/// A sequence of characters.
pub struct String {
    pub(crate) _opaque_type : u64
}

impl Clone for String {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe { transmute_unchecked(self._opaque_type) } }
}

impl String {

    #[lldf_bind_proc::dfdoc(SetVariable/String)]
    #[inline(always)]
    pub fn new() -> Self { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces()
    } }

}

impl From<&str> for String {
    #[inline(always)]
    fn from(value : &str) -> String { unsafe { // TODO: Makes sure empty strings (`""`) still work when they aren't constant.
        DF_ASSERT__ConstantStrToString(value)
    } }
}

impl<T : Into<String>> Add<T> for String {
    type Output = String;
    #[inline(always)]
    fn add(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces(self, rhs.into())
    } }
}

impl String {

    #[lldf_bind_proc::dfdoc(SetVariable/ReplaceString { ReplacementType = AllOccurrences, RegularExpressions = Disable })]
    #[inline(always)]
    pub fn replace<R : Into<String>, W : Into<String>>(&self, replacing : R, with : W) -> String { unsafe {
        DF_ACTION__SetVariable_ReplaceString_ReplacementType_AllOccurrences_RegularExpressions_Disable(self.to_opaque(), replacing.into(), with.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ReplaceString { ReplacementType = FirstOccurrence, RegularExpressions = Disable })]
    #[inline(always)]
    pub fn replace_first<R : Into<String>, W : Into<String>>(&self, replacing : R, with : W) -> String { unsafe {
        DF_ACTION__SetVariable_ReplaceString_ReplacementType_FirstOccurrence_RegularExpressions_Disable(self.to_opaque(), replacing.into(), with.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ReplaceString { ReplacementType = AllOccurrences, RegularExpressions = Enable })]
    #[inline(always)]
    pub fn regex_replace<R : Into<String>, W : Into<String>>(&self, regex : R, with : W) -> String { unsafe {
        DF_ACTION__SetVariable_ReplaceString_ReplacementType_AllOccurrences_RegularExpressions_Enable(self.to_opaque(), regex.into(), with.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ReplaceString { ReplacementType = FirstOccurrence, RegularExpressions = Enable })]
    #[inline(always)]
    pub fn regex_replace_first<R : Into<String>, W : Into<String>>(&self, regex : R, with : W) -> String { unsafe {
        DF_ACTION__SetVariable_ReplaceString_ReplacementType_FirstOccurrence_RegularExpressions_Enable(self.to_opaque(), regex.into(), with.into())
    } }

    // TODO: splice

    #[lldf_bind_proc::dfdoc(SetVariable/SplitString)]
    #[inline(always)]
    pub fn split<D : Into<String>>(&self, delimiter : D) -> List<String> { unsafe {
        DF_ACTION__SetVariable_SplitString(self.to_opaque(), delimiter.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = UPPERCASE })]
    #[inline(always)]
    pub fn uppercase(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseUppercase(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = lowercase })]
    #[inline(always)]
    pub fn lowercase(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseLowercase(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = ProperCase })]
    #[inline(always)]
    pub fn propercase(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcasePropercase(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = InvertCase })]
    #[inline(always)]
    pub fn invert_case(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseInvertcase(self.to_opaque())
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = RandomCase })]
    #[inline(always)]
    pub fn randomise_case(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseRandomcase(self.to_opaque())
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = RandomCase })]
    #[inline(always)]
    pub fn randomize_case(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseRandomcase(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/StringLength)]
    #[inline(always)]
    pub fn len(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_StringLength(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/RepeatString)]
    #[inline(always)]
    pub fn repeat(&self, count : UInt) -> String { unsafe {
        DF_ACTION__SetVariable_RepeatString(self.to_opaque(), count)
    } }

}

// TODO: Parse<Int>

// TODO: Parse<UInt>

// TODO: Parse<Float>

unsafe impl DFValue for String {

    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe { transmute_unchecked(self._opaque_type) } }

    #[inline(always)]
    fn to_string(&self) -> String { self.clone() }

    #[inline(always)]
    fn to_text(&self) -> Text { unsafe {
        DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces(self)
    } }

}


extern "C" {

    fn DF_ACTION__SetVariable_StyledText_InheritStyles_False_TextValueMerging_NoSpaces( ... ) -> Text;

    fn DF_ASSERT__ConstantStrToString( from : &str ) -> String;

    fn DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces( ... ) -> String;
    fn DF_ACTION__SetVariable_ReplaceString_ReplacementType_AllOccurrences_RegularExpressions_Disable( string : DFOpaqueValue, replacing : String, with : String ) -> String;
    fn DF_ACTION__SetVariable_ReplaceString_ReplacementType_FirstOccurrence_RegularExpressions_Disable( string : DFOpaqueValue, replacing : String, with : String ) -> String;
    fn DF_ACTION__SetVariable_ReplaceString_ReplacementType_AllOccurrences_RegularExpressions_Enable( string : DFOpaqueValue, regex : String, with : String ) -> String;
    fn DF_ACTION__SetVariable_ReplaceString_ReplacementType_FirstOccurrence_RegularExpressions_Enable( string : DFOpaqueValue, regex : String, with : String ) -> String;
    fn DF_ACTION__SetVariable_SplitString( string : DFOpaqueValue, delimiter : String ) -> List<String>;
    fn DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseUppercase( string : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseLowercase( string : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcasePropercase( string : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseInvertcase( string : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseRandomcase( string : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_StringLength( string : DFOpaqueValue ) -> UInt;
    fn DF_ACTION__SetVariable_RepeatString( string : DFOpaqueValue, count : UInt ) -> String;

}
