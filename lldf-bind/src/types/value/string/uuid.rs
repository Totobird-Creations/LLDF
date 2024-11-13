use core::mem::transmute_unchecked;

use crate::prelude::*;


/// An entity UUID.
#[repr(transparent)]
pub struct UUID {
    uuid : String
}

impl Clone for UUID {
    #[inline(always)]
    fn clone(&self) -> Self { Self { uuid : self.uuid.clone() } }
}


impl UUID {

    /// Creates a `UUID` from a string.
    /// 
    /// **No checks are done to make sure this uuid is valid.**
    #[inline(always)]
    pub unsafe fn from_string_unchecked<S : Into<String>>(uuid : S) -> Self { Self { uuid : uuid.into() } }

}

impl UUID {

    #[inline(always)]
    pub fn to_string(&self) -> String { unsafe{ transmute_unchecked(self.uuid.to_opaque()) } }

}
