use crate::prelude::*;
use crate::bind::DFOpaqueValue;


/// A particle effect with customisable parameters.
#[derive(Clone)]
pub struct Particle {
    _opaque_type : u8
}


unsafe impl DFValue for Particle {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


extern "C" {

    fn DF_TRANSMUTE__Opaque( from : Particle ) -> DFOpaqueValue;

}


include!("../../bind/particle.rs");

