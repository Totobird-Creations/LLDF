use super::*;
use crate::core::ops::Add;
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
    fn from(value : &str) -> String { unsafe { // TODO: Makes sure empty strings (`""`) still work when they aren't constant.
        DF_ASSERT__ConstantStrToString(transmute_unchecked::<_, &String>(&value).clone())
    } }
}

impl Add<String> for String {
    type Output = String;
    #[inline(always)]
    fn add(self, rhs : String) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces(self, rhs)
    } }
}
impl Add<&str> for String {
    type Output = String;
    #[inline(always)]
    fn add(self, rhs : &str) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces(self, String::from(rhs))
    } }
}

impl String {

    #[lldf_bind_proc::dfdoc(SetVariable/ReplaceString { ReplacementType = AllOccurrences, RegularExpressions = Disable })]
    #[inline(always)]
    pub fn replace(&self, replacing : String, with : String) -> String { unsafe {
        DF_ACTION__SetVariable_ReplaceString_ReplacementType_AllOccurrences_RegularExpressions_Disable(self.clone(), replacing, with)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ReplaceString { ReplacementType = FirstOccurrence, RegularExpressions = Disable })]
    #[inline(always)]
    pub fn replace_first(&self, replacing : String, with : String) -> String { unsafe {
        DF_ACTION__SetVariable_ReplaceString_ReplacementType_FirstOccurrence_RegularExpressions_Disable(self.clone(), replacing, with)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ReplaceString { ReplacementType = AllOccurrences, RegularExpressions = Enable })]
    #[inline(always)]
    pub fn regex_replace(&self, regex : String, with : String) -> String { unsafe {
        DF_ACTION__SetVariable_ReplaceString_ReplacementType_AllOccurrences_RegularExpressions_Enable(self.clone(), regex, with)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ReplaceString { ReplacementType = FirstOccurrence, RegularExpressions = Enable })]
    #[inline(always)]
    pub fn regex_replace_first(&self, regex : String, with : String) -> String { unsafe {
        DF_ACTION__SetVariable_ReplaceString_ReplacementType_FirstOccurrence_RegularExpressions_Enable(self.clone(), regex, with)
    } }

    // TODO: splice

    #[lldf_bind_proc::dfdoc(SetVariable/SplitString)]
    #[inline(always)]
    pub fn split(&self, delimiter : String) -> List<String> { unsafe {
        DF_ACTION__SetVariable_SplitString(self.clone(), delimiter)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = UPPERCASE })]
    #[inline(always)]
    pub fn uppercase(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseUppercase(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = lowercase })]
    #[inline(always)]
    pub fn lowercase(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseLowercase(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = ProperCase })]
    #[inline(always)]
    pub fn propercase(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcasePropercase(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = InvertCase })]
    #[inline(always)]
    pub fn invert_case(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseInvertcase(self.clone())
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = RandomCase })]
    #[inline(always)]
    pub fn randomise_case(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseRandomcase(self.clone())
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetCase { CapitalizationType = RandomCase })]
    #[inline(always)]
    pub fn randomize_case(&self) -> String { unsafe {
        DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseRandomcase(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/StringLength)]
    #[inline(always)]
    pub fn len(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_StringLength(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/RepeatString)]
    #[inline(always)]
    pub fn repeat(&self, count : UInt) -> String { unsafe {
        DF_ACTION__SetVariable_RepeatString(self.clone(), count)
    } }

}

unsafe impl DFValue for String {
    #[inline]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe {
        transmute_unchecked(self._opaque_type.clone())
    } }
}


extern "C" {

    fn DF_ASSERT__ConstantStrToString( from : String ) -> String;

    fn DF_ACTION__SetVariable_String_TextValueMerging_NoSpaces( ... ) -> String;
    fn DF_ACTION__SetVariable_ReplaceString_ReplacementType_AllOccurrences_RegularExpressions_Disable( string : String, replacing : String, with : String ) -> String;
    fn DF_ACTION__SetVariable_ReplaceString_ReplacementType_FirstOccurrence_RegularExpressions_Disable( string : String, replacing : String, with : String ) -> String;
    fn DF_ACTION__SetVariable_ReplaceString_ReplacementType_AllOccurrences_RegularExpressions_Enable( string : String, regex : String, with : String ) -> String;
    fn DF_ACTION__SetVariable_ReplaceString_ReplacementType_FirstOccurrence_RegularExpressions_Enable( string : String, regex : String, with : String ) -> String;
    fn DF_ACTION__SetVariable_SplitString( string : String, delimiter : String ) -> List<String>;
    fn DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseUppercase( string : String ) -> String;
    fn DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseLowercase( string : String ) -> String;
    fn DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcasePropercase( string : String ) -> String;
    fn DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseInvertcase( string : String ) -> String;
    fn DF_ACTION__SetVariable_SetCase_CapitalizationType_SpecialcaseRandomcase( string : String ) -> String;
    fn DF_ACTION__SetVariable_StringLength( string : String ) -> UInt;
    fn DF_ACTION__SetVariable_RepeatString( string : String, count : UInt ) -> String;

}
