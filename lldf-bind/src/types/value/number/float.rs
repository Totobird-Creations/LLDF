use super::*;
use crate::core::float::consts::FRAC_PI_180;
use crate::core::float::consts::FRAC_180_PI;


#[repr(transparent)]
pub struct Float {
    pub(crate) _opaque_type : f64
}
impl Clone for Float { fn clone(&self) -> Self { *self } }
impl Copy for Float {}

impl From<f32  > for Float { #[inline(always)] fn from(value : f32  ) -> Self { Self { _opaque_type : unsafe{ DF_ASSERT__NoOptF64(value as f64) } } } } // TODO: test this
impl From<f64  > for Float { #[inline(always)] fn from(value : f64  ) -> Self { Self { _opaque_type : unsafe{ DF_ASSERT__NoOptF64(value) } } } }
impl From<UInt > for Float { #[inline(always)] fn from(value : UInt ) -> Self { Self { _opaque_type : unsafe{ DF_ASSERT__NoOptF64(value._opaque_type as f64) } } } }
impl From<Int  > for Float { #[inline(always)] fn from(value : Int  ) -> Self { Self { _opaque_type : unsafe{ DF_ASSERT__NoOptF64(value._opaque_type as f64) } } } }

impl<T : Into<Float>> Add<T> for Float {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_Specialcharplus(
            self.to_opaque(),
            rhs.into().to_opaque()
        )
    } }
}

// TODO: Sub

impl<T : Into<Float>> Mul<T> for Float {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_x(
            self.to_opaque(),
            rhs.into().to_opaque()
        )
    } }
}

// TODO: Div

// TODO: Rem

impl Float {

    // TODO: pow

    // TODO: sqrt

    // TODO: cbrt

    // TODO: root

    // TODO: ln

    // TODO: log2

    // TODO: log10

    // TODO: log

    // TODO: abs

    // TODO: clamp

    // TODO: wrap

    // TODO: bounce
    
    // TODO: midpoint

    // TODO: random
    
    // TODO: average

    // TODO: floor

    // TODO: ceil

    // TODO: nearest

    // TODO: trunc

    // TODO: min

    // TODO: max

    // TODO: normaldist

    // TODO: normaldist_fold

    // TODO: sin

    // TODO: arcsin

    // TODO: sinh

    // TODO: cos

    // TODO: arccos

    // TODO: cosh

    // TODO: tan

    // TODO: arctan

    // TODO: tanh

    /// Converts a float in degrees to radians.
    #[inline(always)]
    pub fn to_radians(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_x(self.to_opaque(), Float::from(FRAC_PI_180).to_opaque())
    } }

    /// Converts a float in radians to degrees.
    #[inline(always)]
    pub fn to_degrees(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_x(self.to_opaque(), Float::from(FRAC_180_PI).to_opaque())
    } }

}

unsafe impl DFValue for Float {
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe {
        //DFOpaqueValue { f64 : self._opaque_type }
        transmute_unchecked(self._opaque_type)
    } }
}
impl Into<Float> for Float {
    #[inline(always)]
    fn into(self) -> Float { self }
}


extern "C" {

    fn DF_ASSERT__NoOptF64( from : f64 ) -> f64;

    fn DF_ACTION__SetVariable_Specialcharplus( left : DFOpaqueValue, right : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_x( left : DFOpaqueValue, right : DFOpaqueValue ) -> Float;

}
