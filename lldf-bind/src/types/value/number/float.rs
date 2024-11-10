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

unsafe impl DFValue for Float {
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Float_Opaque(self)
    } }
}
impl Into<Float> for Float {
    #[inline(always)]
    fn into(self) -> Float { self }
}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__Float_Opaque( from : Float ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_Specialcharplus( left : Float, right : Float ) -> Float;
    fn DF_ACTION__SetVariable_x( left : Float, right : Float ) -> Float;

}
