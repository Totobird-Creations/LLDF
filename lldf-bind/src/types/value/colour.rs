use crate::prelude::*;
use crate::bind::DFOpaqueValue;


#[derive(Clone)]
#[repr(transparent)]
pub struct RGB {
    hexcode : String
}


unsafe impl DFValue for RGB {
    #[inline(always)]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        self.hexcode.to_opaque()
    } }
}
