use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


/// A particle effect with customisable parameters.
pub struct Particle {
    _opaque_type : u64
}

impl Clone for Particle {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe { transmute_unchecked(self._opaque_type) } }
}

impl Particle {

    #[inline(always)]
    pub fn new(kind : ParticleKind) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleType(Self::cloud().to_opaque(), kind.to_string())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleType)]
    #[inline(always)]
    pub fn kind(&self) -> ParticleKind { unsafe {
        ParticleKind::from_string_unchecked(DF_ACTION__SetVariable_GetParticleType(self.to_opaque()))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleAmount)]
    #[inline(always)]
    pub fn amount(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_GetParticleAmount(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleAmount)]
    #[inline(always)]
    pub fn with_amount<U : Into<UInt>>(&self, amount : U) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleAmount(self.to_opaque(), amount.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleSprd)]
    #[inline(always)]
    pub fn spread(&self) -> (Float, Float) { unsafe {
        (
            DF_ACTION__SetVariable_GetParticleSprd_Spread_Horizontal(self.to_opaque()),
            DF_ACTION__SetVariable_GetParticleSprd_Spread_Vertical(self.to_opaque())
        )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleSprd)]
    #[inline(always)]
    pub fn with_spread<H : Into<Float>, V : Into<Float>>(&self, horizontal : H, vertical : V) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleSprd(self.to_opaque(), horizontal.into(), vertical.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleSize)]
    #[inline(always)]
    pub fn size(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetParticleSize(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleSize)]
    #[inline(always)]
    pub fn with_size<F : Into<Float>, V : Into<Float>>(&self, size : F, variation_percent : V) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleSize(self.to_opaque(), size.into(), variation_percent.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleMat)]
    /// ##### Unsafe
    /// - **Does not do a property check**
    #[inline(always)]
    pub unsafe fn material_unchecked(&self) -> String { unsafe {
        DF_ACTION__SetVariable_GetParticleMat(self.to_opaque())
    } }

    #[deprecated = "GetParticleMat is currently broken on DiamondFire"]
    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleMat)]
    #[inline(always)]
    pub fn material(&self) -> Option<String> { unsafe { // TODO: Is GetParticleMat not working?
        let material = DF_ACTION__SetVariable_GetParticleMat(self.to_opaque());
        DF_ACTION__IfVariable_VarIsType_VariableType_String(material.to_opaque());
        let success = DF_TEMPVAR();
        DF_BRACKET__Normal_Open();
        DF_ACTION__SetVariable_Specialcharequals(success, UInt::from(1usize).to_opaque());
        DF_BRACKET__Normal_Close();
        DF_ELSE();
        DF_BRACKET__Normal_Open();
        DF_ACTION__SetVariable_Specialcharequals(success, UInt::from(0usize).to_opaque());
        DF_BRACKET__Normal_Close();
        return if (transmute_unchecked::<_, UInt>(success) == UInt::from(0usize)) { None } else { Some(material) };
    } }

    #[deprecated = "GetParticleMat is currently broken on DiamondFire"]
    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleMat)]
    #[inline(always)]
    pub fn with_material(&self, material : Item) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleMat(self.to_opaque(), material)
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleColor)]
    #[inline(always)]
    pub fn colour(&self) -> Colour { unsafe {
        Colour::from_hexcode_unchecked(DF_ACTION__SetVariable_GetParticleColor(self.to_opaque()))
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleColor)]
    #[inline(always)]
    pub fn color(&self) -> Color { unsafe {
        Color::from_hexcode_unchecked(DF_ACTION__SetVariable_GetParticleColor(self.to_opaque()))
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleColor)]
    #[inline(always)]
    pub fn with_colour(&self, colour : Colour, variation_percent : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleColor(self.to_opaque(), colour.hexcode(), variation_percent)
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleColor)]
    #[inline(always)]
    pub fn with_color(&self, color : Color, variation_percent : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleColor(self.to_opaque(), color.hexcode(), variation_percent)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleOpac)]
    #[inline(always)]
    pub fn opacity(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetParticleOpac(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleOpac)]
    #[inline(always)]
    pub fn with_opacity<F : Into<Float>>(&self, opacity : F) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleOpac(self.to_opaque(), opacity.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleMotion)]
    #[inline(always)]
    pub fn motion(&self) -> Vector<3> { unsafe {
        DF_ACTION__SetVariable_GetParticleMotion(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleMotion)]
    #[inline(always)]
    pub fn with_motion<V : Into<Float>>(&self, motion : Vector<3>, variation_percent : V) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleMotion(self.to_opaque(), motion, variation_percent.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleRoll)]
    #[inline(always)]
    pub fn roll(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetParticleRoll(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleRoll)]
    #[inline(always)]
    pub fn with_roll<F : Into<Float>>(&self, roll : F) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleRoll(self.to_opaque(), roll.into())
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleFade)]
    #[inline(always)]
    pub fn fade_colour(&self) -> Colour { unsafe {
        Colour::from_hexcode_unchecked(DF_ACTION__SetVariable_GetParticleFade(self.to_opaque()))
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleFade)]
    #[inline(always)]
    pub fn fade_color(&self) -> Color { unsafe {
        Color::from_hexcode_unchecked(DF_ACTION__SetVariable_GetParticleFade(self.to_opaque()))
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleFade)]
    #[inline(always)]
    pub fn with_fade_colour(&self, colour : Colour) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleFade(self.to_opaque(), colour.hexcode())
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleFade)]
    #[inline(always)]
    pub fn with_fade_color(&self, color : Color, variation_percent : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleFade(self.to_opaque(), color.hexcode(), variation_percent)
    } }

}


unsafe impl DFValue for Particle {

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

    fn DF_TEMPVAR() -> DFOpaqueValue;
    fn DF_ACTION__IfVariable_VarIsType_VariableType_String( value : DFOpaqueValue ) -> ();
    fn DF_BRACKET__Normal_Open() -> ();
    fn DF_BRACKET__Normal_Close() -> ();
    fn DF_ELSE() -> ();
    fn DF_ACTION__SetVariable_Specialcharequals( variable : DFOpaqueValue, value : DFOpaqueValue ) -> ();

    fn DF_ACTION__SetVariable_SetParticleType( particle : DFOpaqueValue, kind : String ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleType( particle : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_GetParticleAmount( particle : DFOpaqueValue ) -> UInt;
    fn DF_ACTION__SetVariable_SetParticleAmount( particle : DFOpaqueValue, amount : UInt ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleSprd_Spread_Horizontal( particle : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_GetParticleSprd_Spread_Vertical( particle : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_SetParticleSprd( particle : DFOpaqueValue, horizontal : Float, vertical : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleSize( particle : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_SetParticleSize( particle : DFOpaqueValue, size : Float, variation_percent : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleMat( particle : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_SetParticleMat( particle : DFOpaqueValue, material : Item ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleColor( particle : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_SetParticleColor( particle : DFOpaqueValue, colour : String, variation_percent : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleOpac( particle : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_SetParticleOpac( particle : DFOpaqueValue, opacity : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleMotion( particle : DFOpaqueValue ) -> Vector<3>;
    fn DF_ACTION__SetVariable_SetParticleMotion( particle : DFOpaqueValue, motion : Vector<3>, variation_percent : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleRoll( particle : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_SetParticleRoll( particle : DFOpaqueValue, roll : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleFade( particle : DFOpaqueValue ) -> String;
    fn DF_ACTION__SetVariable_SetParticleFade( particle : DFOpaqueValue, colour : String ) -> Particle;

}


include!("../../bind/particle.rs");

