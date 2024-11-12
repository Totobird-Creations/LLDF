use super::*;


#[repr(transparent)]
pub struct Float {
    _opaque_type : f32
}
impl Clone for Float { fn clone(&self) -> Self { *self } }
impl Copy for Float {}

impl From<f32> for Float { #[inline(always)] fn from(value : f32) -> Self { Self { _opaque_type : value } } }
impl From<f64> for Float { #[inline(always)] fn from(value : f64) -> Self { Self { _opaque_type : value as f32 } } } // TODO: test this
impl From<UInt> for Float { #[inline(always)] fn from(value : UInt) -> Self { Self { _opaque_type : value._opaque_type as f32 } } }

impl<T : Into<Float>> Add<T> for Float {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_Specialcharplus(
            self,
            rhs.into()
        )
    } }
}

// TODO: Sub

impl<T : Into<Float>> Mul<T> for Float {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs : T) -> Self::Output { unsafe {
        DF_ACTION__SetVariable_x(
            self,
            rhs.into()
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

}

unsafe impl DFValue for Float {
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe {
        transmute_unchecked(self._opaque_type.clone())
    } }
}
impl Into<Float> for Float {
    #[inline(always)]
    fn into(self) -> Float { self }
}


extern "C" {

    fn DF_ACTION__SetVariable_Specialcharplus( left : Float, right : Float ) -> Float;
    fn DF_ACTION__SetVariable_x( left : Float, right : Float ) -> Float;

}
