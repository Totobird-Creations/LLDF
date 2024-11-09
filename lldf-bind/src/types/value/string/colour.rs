use super::*;


#[derive(Clone)]
#[repr(transparent)]
pub struct Colour {
    hexcode : String
}


unsafe impl DFValue for Colour {
    #[inline(always)]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        self.hexcode.to_opaque()
    } }
}
