use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


/// A potion effect with amplifier and duration.
pub struct Potion {
    _opaque_type : u64
}

impl Clone for Potion {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe { transmute_unchecked(self._opaque_type) } }
}

impl Potion {

    #[inline(always)]
    pub fn new(kind : PotionKind) -> Potion { unsafe {
        DF_ACTION__SetVariable_SetPotionType(Self::speed().to_opaque(), kind.to_string())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetPotionType)]
    #[inline(always)]
    pub fn kind(&self) -> PotionKind { unsafe {
        PotionKind::from_string_unchecked(DF_ACTION__SetVariable_GetPotionType(self.to_opaque()))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetPotionDur)]
    #[inline(always)]
    pub fn duration(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_GetPotionDur(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetPotionDur)]
    #[inline(always)]
    pub fn with_duration<U : Into<UInt>>(&self, duration_ticks : U) -> Potion { unsafe {
        DF_ACTION__SetVariable_SetPotionDur(self.to_opaque(), duration_ticks.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetPotionAmp)]
    #[inline(always)]
    pub fn amplifier(&self) -> UInt { unsafe {
        let out = DF_ACTION__SetVariable_GetPotionAmp(self.to_opaque());
        transmute_unchecked(DF_ACTION__SetVariable_Specialcharminus(out.to_opaque(), UInt::from(1usize).to_opaque()))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetPotionAmp)]
    #[inline(always)]
    pub fn with_amplifier<U : Into<UInt>>(&self, amplifier : U) -> Potion { unsafe {
        let amplifier = transmute_unchecked(DF_ACTION__SetVariable_Specialcharplus(amplifier.into().to_opaque(), UInt::from(1usize).to_opaque()));
        DF_ACTION__SetVariable_SetPotionAmp(self.to_opaque(), amplifier)
    } }

}


unsafe impl DFValue for Potion {

    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe { transmute_unchecked(self._opaque_type) } }

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

    fn DF_ACTION__SetVariable_Specialcharplus( a : DFOpaqueValue, b : DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_Specialcharminus( a : DFOpaqueValue, b : DFOpaqueValue ) -> DFOpaqueValue;


    fn DF_ACTION__SetVariable_SetPotionType( potion : DFOpaqueValue, kind : String ) -> Potion;
    fn DF_ACTION__SetVariable_GetPotionType( potion : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_GetPotionDur( potion : DFOpaqueValue ) -> UInt;
    fn DF_ACTION__SetVariable_SetPotionDur( potion : DFOpaqueValue, duration_ticks : UInt ) -> Potion;
    fn DF_ACTION__SetVariable_GetPotionAmp( potion : DFOpaqueValue ) -> UInt;
    fn DF_ACTION__SetVariable_SetPotionAmp( potion : DFOpaqueValue, amplifier : UInt ) -> Potion;

}


include!("../../bind/potion.rs");

