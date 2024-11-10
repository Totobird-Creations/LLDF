use crate::prelude::*;
use crate::bind::DFOpaqueValue;


/// A Minecraft sound or a custom sound from a resource pack.
#[derive(Clone)]
pub struct Sound {
    _opaque_type : u8
}

impl Sound {

    // TODO: name

    #[lldf_bind_proc::dfdoc(SetVariable/GetCustomSound)]
    #[inline(always)]
    pub fn key(&self) -> String { unsafe {
        DF_ACTION__SetVariable_GetCustomSound(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetSoundVolume)]
    #[inline(always)]
    pub fn volume(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetSoundVolume(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetSoundPitch)]
    #[inline(always)]
    pub fn pitch(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetSoundPitch(self.clone())
    } }

    // TODO: variant

}

impl Sound {

    #[lldf_bind_proc::dfdoc(SetVariable/SetCustomSound)]
    #[inline(always)]
    pub fn custom<S : Into<String>>(key : S) -> Self { unsafe {
        DF_ACTION__SetVariable_SetCustomSound(Sound::block_stone_button_click_on(), key.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetSoundVolume)]
    #[inline(always)]
    pub fn with_volume<F : Into<Float>>(&self, volume : F) -> Self { unsafe {
        DF_ACTION__SetVariable_SetSoundVolume(self.clone(), volume.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetSoundPitch)]
    #[inline(always)]
    pub fn with_pitch<F : Into<Float>>(&self, pitch : F) -> Self { unsafe {
        DF_ACTION__SetVariable_SetSoundPitch(self.clone(), pitch.into())
    } }

    // TODO: with_variant

}


unsafe impl DFValue for Sound {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__Opaque( from : Sound ) -> DFOpaqueValue;


    fn DF_ACTION__SetVariable_SetCustomSound( sound : Sound, key : String ) -> Sound;
    fn DF_ACTION__SetVariable_GetCustomSound( sound : Sound ) -> String;
    fn DF_ACTION__SetVariable_SetSoundVolume( sound : Sound, volume : Float ) -> Sound;
    fn DF_ACTION__SetVariable_GetSoundVolume( sound : Sound ) -> Float;
    fn DF_ACTION__SetVariable_SetSoundPitch( sound : Sound, pitch : Float ) -> Sound;
    fn DF_ACTION__SetVariable_GetSoundPitch( sound : Sound ) -> Float;

}


include!("../../bind/sound.rs");
