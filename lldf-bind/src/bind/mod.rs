use crate::prelude::*;


/// A value returned by a DiamondFire operation.
#[derive(Clone)]
pub struct DFOpaqueValue {
    _opaque_type : u8
}

unsafe impl DFValue for DFOpaqueValue {
    unsafe fn to_opaque(self) -> DFOpaqueValue { self }
}
