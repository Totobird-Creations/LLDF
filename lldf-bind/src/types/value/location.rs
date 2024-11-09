use crate::prelude::*;
use crate::bind::DFOpaqueValue;


#[derive(Clone)]
pub struct Location {
    _opaque_type : u8
}

unsafe impl DFValue for Location {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


#[allow(clashing_extern_declarations)]
extern "C" {

    fn DF_TRANSMUTE__Opaque( from : Location ) -> DFOpaqueValue;

}
