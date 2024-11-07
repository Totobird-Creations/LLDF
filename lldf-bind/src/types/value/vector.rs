use crate::prelude::*;
use crate::bind::DFOpaqueValue;


#[derive(Clone)]
pub struct Vector {
    _opaque_type : u8
}

impl Vector {

    #[inline(always)]
    pub fn new(x : Float, y : Float, z : Float) -> Vector { unsafe {
        DF_ACTION__SetVariable_Vector(x, y, z)
    } }

}

unsafe impl DFValue for Vector {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__Opaque( from : Vector ) -> DFOpaqueValue;

    fn DF_ACTION__SetVariable_Vector( x : Float, y : Float, z : Float ) -> Vector;

}
