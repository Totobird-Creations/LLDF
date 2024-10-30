use crate::prelude::*;


#[derive(Clone)]
pub struct Item {
    _opaque_type : u8
}


unsafe impl DFValue for Item {}
