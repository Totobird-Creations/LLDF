use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


/// A vanilla sound or a custom sound from a resource pack.
#[derive(Clone)]
pub struct Sound {
    _opaque_type : u8
}

impl Sound {

    // TODO: name

    #[lldf_bind_proc::dfdoc(SetVariable/GetCustomSound)]
    #[inline(always)]
    pub fn key(&self) -> String { unsafe {
        DF_ACTION__SetVariable_GetCustomSound(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetSoundVolume)]
    #[inline(always)]
    pub fn volume(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetSoundVolume(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetSoundPitch)]
    #[inline(always)]
    pub fn pitch(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetSoundPitch(self.to_opaque())
    } }

    // TODO: variant

}

impl Sound {

    #[lldf_bind_proc::dfdoc(SetVariable/SetCustomSound)]
    #[inline(always)]
    pub fn custom<S : Into<String>>(key : S) -> Self { unsafe {
        DF_ACTION__SetVariable_SetCustomSound(Sound::block_stone_button_click_on().to_opaque(), key.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetSoundVolume)]
    #[inline(always)]
    pub fn with_volume<F : Into<Float>>(&self, volume : F) -> Self { unsafe {
        DF_ACTION__SetVariable_SetSoundVolume(self.to_opaque(), volume.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetSoundPitch)]
    #[inline(always)]
    pub fn with_pitch<F : Into<Float>>(&self, pitch : F) -> Self { unsafe {
        DF_ACTION__SetVariable_SetSoundPitch(self.to_opaque(), pitch.into())
    } }

    // TODO: with_variant

}


unsafe impl DFValue for Sound {
    #[inline]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe {
        transmute_unchecked(self._opaque_type.clone())
    } }
}


extern "C" {

    fn DF_ACTION__SetVariable_SetCustomSound( sound : DFOpaqueValue, key : String ) -> Sound;
    fn DF_ACTION__SetVariable_GetCustomSound( sound : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_SetSoundVolume( sound : DFOpaqueValue, volume : Float ) -> Sound;
    fn DF_ACTION__SetVariable_GetSoundVolume( sound : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_SetSoundPitch( sound : DFOpaqueValue, pitch : Float ) -> Sound;
    fn DF_ACTION__SetVariable_GetSoundPitch( sound : DFOpaqueValue ) -> Float;

}


include!("../../bind/sound.rs");
