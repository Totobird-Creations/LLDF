use crate::prelude::*;


#[derive(Clone, Copy)]
pub struct Item {
    _opaque_type : u8
}


unsafe impl DFValue for Item {}
