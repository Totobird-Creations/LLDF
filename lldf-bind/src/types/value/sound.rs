use crate::prelude::*;
use crate::bind::DFOpaqueValue;


#[derive(Clone)]
pub struct Sound {
    _opaque_type : u8
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

}


include!("../../bind/sounds.rs");
