use super::*;


#[repr(transparent)]
pub struct Float {
    _opaque_type : f32
}
impl Clone for Float { fn clone(&self) -> Self { *self } }
impl Copy for Float {}

impl From<f32> for Float { #[inline(always)] fn from(value : f32) -> Self { Self { _opaque_type : value } } }
impl From<f64> for Float { #[inline(always)] fn from(value : f64) -> Self { Self { _opaque_type : value as f32 } } } // TODO: test this

unsafe impl DFValue for Float {
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Float_Opaque(self)
    } }
}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__Float_Opaque( from : Float ) -> DFOpaqueValue;

}
