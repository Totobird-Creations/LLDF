use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


/// A vanilla sound or a custom sound from a resource pack.
pub struct Sound {
    _opaque_type : u64
}

impl Clone for Sound {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe { transmute_unchecked(self._opaque_type) } }
}

impl Sound {

    #[inline(always)]
    pub fn new(kind : SoundKind) -> Sound { unsafe {
        DF_ACTION__SetVariable_SetSoundType(Self::stone_button_click_on().to_opaque(), kind.to_string())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetSoundType)]
    #[inline(always)]
    pub unsafe fn kind_unchecked(&self) -> SoundKind { unsafe {
        SoundKind::from_string_unchecked(DF_ACTION__SetVariable_GetSoundType(self.to_opaque()))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleMat)]
    #[inline(always)]
    pub fn kind(&self) -> Option<SoundKind> { unsafe {
        let kind = DF_ACTION__SetVariable_GetSoundType(self.to_opaque());
        DF_ACTION__IfVariable_VarIsType_VariableType_String(kind.to_opaque());
        let success = DF_TEMPVAR();
        DF_BRACKET__Normal_Open();
        DF_ACTION__SetVariable_Specialcharequals(success, UInt::from(1usize).to_opaque());
        DF_BRACKET__Normal_Close();
        DF_ELSE();
        DF_BRACKET__Normal_Open();
        DF_ACTION__SetVariable_Specialcharequals(success, UInt::from(0usize).to_opaque());
        DF_BRACKET__Normal_Close();
        return if (transmute_unchecked::<_, UInt>(success) == UInt::from(0usize)) { None } else { Some(SoundKind::from_string_unchecked(kind)) };
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCustomSound)]
    #[inline(always)]
    pub fn custom<S : Into<String>>(key : S) -> Self { unsafe {
        DF_ACTION__SetVariable_SetCustomSound(Sound::stone_button_click_on().to_opaque(), key.into())
    } }

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

    #[lldf_bind_proc::dfdoc(SetVariable/SetSoundVolume)]
    #[inline(always)]
    pub fn with_volume<F : Into<Float>>(&self, volume : F) -> Self { unsafe {
        DF_ACTION__SetVariable_SetSoundVolume(self.to_opaque(), volume.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetSoundPitch)]
    #[inline(always)]
    pub fn pitch(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetSoundPitch(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetSoundPitch)]
    #[inline(always)]
    pub fn with_pitch<F : Into<Float>>(&self, pitch : F) -> Self { unsafe {
        DF_ACTION__SetVariable_SetSoundPitch(self.to_opaque(), pitch.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetSoundVariant)]
    #[inline(always)]
    pub unsafe fn variant_unchecked(&self) -> String { unsafe {
        DF_ACTION__SetVariable_GetSoundVariant(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetSoundVariant)]
    #[inline(always)]
    pub fn variant(&self) -> Option<String> { unsafe {
        let variant = DF_ACTION__SetVariable_GetSoundVariant(self.to_opaque());
        DF_ACTION__IfVariable_VarIsType_VariableType_String(variant.to_opaque());
        let success = DF_TEMPVAR();
        DF_BRACKET__Normal_Open();
        DF_ACTION__SetVariable_Specialcharequals(success, UInt::from(1usize).to_opaque());
        DF_BRACKET__Normal_Close();
        DF_ELSE();
        DF_BRACKET__Normal_Open();
        DF_ACTION__SetVariable_Specialcharequals(success, UInt::from(0usize).to_opaque());
        DF_BRACKET__Normal_Close();
        return if (transmute_unchecked::<_, UInt>(success) == UInt::from(0usize)) { None } else { Some(variant) };
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetSoundVariant)]
    #[inline(always)]
    pub fn with_variant<S : Into<String>>(&self, variant : S) -> Self { unsafe {
        DF_ACTION__SetVariable_SetSoundVariant(self.to_opaque(), variant.into())
    } }

}


unsafe impl DFValue for Sound {
    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe { transmute_unchecked(self._opaque_type) } }
}


extern "C" {

    fn DF_TEMPVAR() -> DFOpaqueValue;
    fn DF_ACTION__IfVariable_VarIsType_VariableType_String( value : DFOpaqueValue ) -> ();
    fn DF_BRACKET__Normal_Open() -> ();
    fn DF_BRACKET__Normal_Close() -> ();
    fn DF_ELSE() -> ();
    fn DF_ACTION__SetVariable_Specialcharequals( variable : DFOpaqueValue, value : DFOpaqueValue ) -> ();

    fn DF_ACTION__SetVariable_SetSoundType( sound : DFOpaqueValue, kind : String ) -> Sound;
    fn DF_ACTION__SetVariable_GetSoundType( sound : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_SetCustomSound( sound : DFOpaqueValue, key : String ) -> Sound;
    fn DF_ACTION__SetVariable_GetCustomSound( sound : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_SetSoundVolume( sound : DFOpaqueValue, volume : Float ) -> Sound;
    fn DF_ACTION__SetVariable_GetSoundVolume( sound : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_SetSoundPitch( sound : DFOpaqueValue, pitch : Float ) -> Sound;
    fn DF_ACTION__SetVariable_GetSoundPitch( sound : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_SetSoundVariant( sound : DFOpaqueValue, variant : String ) -> Sound;
    fn DF_ACTION__SetVariable_GetSoundVariant( sound : DFOpaqueValue ) -> String;

}


include!("../../bind/sound.rs");
