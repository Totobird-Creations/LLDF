use crate::prelude::*;
use crate::core::mem::transmute_unchecked;


#[derive(Clone, Copy)]
pub struct String {
    _opaque_type : u8
}

impl Into<String> for &str {
    fn into(self) -> String { unsafe{ *transmute_unchecked::<_, &_>(&self) } }
}

unsafe impl DFValue for String {}
