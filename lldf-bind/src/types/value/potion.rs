use crate::prelude::*;
use crate::bind::DFOpaqueValue;


/// A potion effect with amplifier and duration.
#[derive(Clone)]
pub struct Potion {
    _opaque_type : u8
}

impl Potion {

    #[lldf_bind_proc::dfdoc(SetVariable/GetPotionDur)]
    #[inline(always)]
    pub fn duration(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_GetPotionDur(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetPotionAmp)]
    #[inline(always)]
    pub fn amplifier(&self) -> UInt { unsafe {
        let out = DF_ACTION__SetVariable_GetPotionAmp(self.clone());
        DF_TRANSMUTE__Opaque_UInt(DF_ACTION__SetVariable_Specialcharminus(out.to_opaque(), UInt::from(1usize).to_opaque()))
    } }

}

impl Potion {

    #[lldf_bind_proc::dfdoc(SetVariable/SetPotionDur)]
    #[inline(always)]
    pub fn with_duration<U : Into<UInt>>(&self, duration_ticks : U) -> Self { unsafe {
        DF_ACTION__SetVariable_SetPotionDur(self.clone(), duration_ticks.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetPotionAmp)]
    #[inline(always)]
    pub fn with_amplifier<U : Into<UInt>>(&self, amplifier : U) -> Self { unsafe {
        let amplifier = DF_TRANSMUTE__Opaque_UInt(DF_ACTION__SetVariable_Specialcharplus(amplifier.into().to_opaque(), UInt::from(1usize).to_opaque()));
        DF_ACTION__SetVariable_SetPotionAmp(self.clone(), amplifier)
    } }

}


unsafe impl DFValue for Potion {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


extern "C" {

    fn DF_TRANSMUTE__Opaque( from : Potion ) -> DFOpaqueValue;
    fn DF_TRANSMUTE__Opaque_UInt( from : DFOpaqueValue ) -> UInt;
    fn DF_ACTION__SetVariable_Specialcharplus( a : DFOpaqueValue, b : DFOpaqueValue ) -> DFOpaqueValue;
    fn DF_ACTION__SetVariable_Specialcharminus( a : DFOpaqueValue, b : DFOpaqueValue ) -> DFOpaqueValue;


    fn DF_ACTION__SetVariable_SetPotionDur( potion : Potion, duration_ticks : UInt ) -> Potion;
    fn DF_ACTION__SetVariable_GetPotionDur( potion : Potion ) -> UInt;
    fn DF_ACTION__SetVariable_SetPotionAmp( potion : Potion, amplifier : UInt ) -> Potion;
    fn DF_ACTION__SetVariable_GetPotionAmp( potion : Potion ) -> UInt;

}


include!("../../bind/potion.rs");

