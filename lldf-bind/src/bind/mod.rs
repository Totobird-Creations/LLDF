use crate::prelude::*;
use crate::core::mem::transmute_unchecked;


/// A value returned by a DiamondFire operation.
#[derive(Clone, Copy)]
pub struct DFOpaqueValue {
    _opaque_type : u8
}

unsafe impl DFValue for DFOpaqueValue {
    unsafe fn to_opaque(&self) -> DFOpaqueValue { transmute_unchecked(self._opaque_type.clone()) }
}
