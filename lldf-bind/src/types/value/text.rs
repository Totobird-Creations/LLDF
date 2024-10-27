use crate::prelude::*;


#[derive(Clone, Copy)]
pub struct Text {
    _opaque_type : u8
}

impl Into<Text> for &str { fn into(self) -> Text { unsafe { *crate::core::mem::transmute_unchecked::<_, &_>(&self) } } }
impl Into<Text> for String { fn into(self) -> Text { unsafe { *crate::core::mem::transmute_unchecked::<_, &_>(&self) } } }

unsafe impl DFValue for Text {}
