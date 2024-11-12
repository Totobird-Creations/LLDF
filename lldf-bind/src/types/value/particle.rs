use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


/// A particle effect with customisable parameters.
#[derive(Clone)]
pub struct Particle {
    _opaque_type : u8
}


impl Particle {

    #[inline(always)]
    pub fn new(kind : ParticleKind) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleType(Self::cloud(), kind.to_string())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleType)]
    #[inline(always)]
    pub fn kind(&self) -> ParticleKind { unsafe {
        ParticleKind::from_string_unchecked(DF_ACTION__SetVariable_GetParticleType(self.clone()))
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleAmount)]
    #[inline(always)]
    pub fn amount(&self) -> UInt { unsafe {
        DF_ACTION__SetVariable_GetParticleAmount(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleAmount)]
    #[inline(always)]
    pub fn with_amount(&self, amount : UInt) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleAmount(self.clone(), amount)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleSprd)]
    #[inline(always)]
    pub fn spread(&self) -> (Float, Float) { unsafe {
        (
            DF_ACTION__SetVariable_GetParticleSprd_Spread_Horizontal(self.clone()),
            DF_ACTION__SetVariable_GetParticleSprd_Spread_Vertical(self.clone())
        )
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleSprd)]
    #[inline(always)]
    pub fn with_spread(&self, horizontal : Float, vertical : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleSprd(self.clone(), horizontal, vertical)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleSize)]
    #[inline(always)]
    pub fn size(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetParticleSize(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleSize)]
    #[inline(always)]
    pub fn with_size(&self, size : Float, variation_percent : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleSize(self.clone(), size, variation_percent)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleMat)]
    #[inline(always)]
    pub fn material(&self) -> String { unsafe {
        DF_ACTION__SetVariable_GetParticleMat(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleMat)]
    #[inline(always)]
    pub fn with_material(&self, material : Item) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleMat(self.clone(), material)
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleColor)]
    #[inline(always)]
    pub fn colour(&self) -> Colour { unsafe {
        Colour::from_hexcode_unchecked(DF_ACTION__SetVariable_GetParticleColor(self.clone()))
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleColor)]
    #[inline(always)]
    pub fn color(&self) -> Color { unsafe {
        Color::from_hexcode_unchecked(DF_ACTION__SetVariable_GetParticleColor(self.clone()))
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleColor)]
    #[inline(always)]
    pub fn with_colour(&self, colour : Colour, variation_percent : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleColor(self.clone(), colour.hexcode(), variation_percent)
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleColor)]
    #[inline(always)]
    pub fn with_color(&self, color : Color, variation_percent : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleColor(self.clone(), color.hexcode(), variation_percent)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleOpac)]
    #[inline(always)]
    pub fn opacity(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetParticleOpac(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleOpac)]
    #[inline(always)]
    pub fn with_opacity(&self, opacity : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleOpac(self.clone(), opacity)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleMotion)]
    #[inline(always)]
    pub fn motion(&self) -> Vector<3> { unsafe {
        DF_ACTION__SetVariable_GetParticleMotion(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleMotion)]
    #[inline(always)]
    pub fn with_motion(&self, motion : Vector<3>, variation_percent : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleMotion(self.clone(), motion, variation_percent)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleRoll)]
    #[inline(always)]
    pub fn roll(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetParticleRoll(self.clone())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleRoll)]
    #[inline(always)]
    pub fn with_roll(&self, roll : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleRoll(self.clone(), roll)
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleFade)]
    #[inline(always)]
    pub fn fade_colour(&self) -> Colour { unsafe {
        Colour::from_hexcode_unchecked(DF_ACTION__SetVariable_GetParticleFade(self.clone()))
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/GetParticleFade)]
    #[inline(always)]
    pub fn fade_color(&self) -> Color { unsafe {
        Color::from_hexcode_unchecked(DF_ACTION__SetVariable_GetParticleFade(self.clone()))
    } }

    #[cfg(any(not(feature = "en_us"), doc))]
    #[doc(cfg(not(feature = "en_us")))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleFade)]
    #[inline(always)]
    pub fn with_fade_colour(&self, colour : Colour) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleFade(self.clone(), colour.hexcode())
    } }
    #[cfg(any(feature = "en_us", doc))]
    #[doc(cfg(feature = "en_us"))]
    #[lldf_bind_proc::dfdoc(SetVariable/SetParticleFade)]
    #[inline(always)]
    pub fn with_fade_color(&self, color : Color, variation_percent : Float) -> Particle { unsafe {
        DF_ACTION__SetVariable_SetParticleFade(self.clone(), color.hexcode(), variation_percent)
    } }

}


unsafe impl DFValue for Particle {
    #[inline]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe {
        transmute_unchecked(self._opaque_type.clone())
    } }
}


extern "C" {

    fn DF_ACTION__SetVariable_SetParticleType( particle : Particle, kind : String ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleType( particle : Particle ) -> String;
    fn DF_ACTION__SetVariable_GetParticleAmount( particle : Particle ) -> UInt;
    fn DF_ACTION__SetVariable_SetParticleAmount( particle : Particle, amount : UInt ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleSprd_Spread_Horizontal( particle : Particle ) -> Float;
    fn DF_ACTION__SetVariable_GetParticleSprd_Spread_Vertical( particle : Particle ) -> Float;
    fn DF_ACTION__SetVariable_SetParticleSprd( particle : Particle, horizontal : Float, vertical : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleSize( particle : Particle ) -> Float;
    fn DF_ACTION__SetVariable_SetParticleSize( particle : Particle, size : Float, variation_percent : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleMat( particle : Particle ) -> String;
    fn DF_ACTION__SetVariable_SetParticleMat( particle : Particle, material : Item ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleColor( particle : Particle ) -> String;
    fn DF_ACTION__SetVariable_SetParticleColor( particle : Particle, colour : String, variation_percent : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleOpac( particle : Particle ) -> Float;
    fn DF_ACTION__SetVariable_SetParticleOpac( particle : Particle, opacity : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleMotion( particle : Particle ) -> Vector<3>;
    fn DF_ACTION__SetVariable_SetParticleMotion( particle : Particle, motion : Vector<3>, variation_percent : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleRoll( particle : Particle ) -> Float;
    fn DF_ACTION__SetVariable_SetParticleRoll( particle : Particle, roll : Float ) -> Particle;
    fn DF_ACTION__SetVariable_GetParticleFade( particle : Particle ) -> String;
    fn DF_ACTION__SetVariable_SetParticleFade( particle : Particle, colour : String ) -> Particle;

}


include!("../../bind/particle.rs");

