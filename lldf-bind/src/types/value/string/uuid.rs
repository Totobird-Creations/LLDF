use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


/// An entity UUID.
#[repr(transparent)]
pub struct Uuid {
    uuid : String
}

impl Clone for Uuid {
    #[inline(always)]
    fn clone(&self) -> Self { Self { uuid : self.uuid.clone() } }
}

unsafe impl DFValue for Uuid {

    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { self.uuid.to_opaque() }

    #[inline(always)]
    fn to_string(&self) -> String { self.uuid.clone() }

    #[inline(always)]
    fn to_text(&self) -> Text { self.uuid.to_text() }

}


impl Uuid {

    /// Creates a `UUID` from a string.
    /// 
    /// **No checks are done to make sure this uuid is valid.**
    #[inline(always)]
    pub unsafe fn from_string_unchecked<S : Into<String>>(uuid : S) -> Self { Self { uuid : uuid.into() } }

}

impl Uuid {

    #[inline(always)]
    pub fn to_string(&self) -> String { unsafe{ transmute_unchecked(self.uuid.to_opaque()) } }

}
