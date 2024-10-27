use crate::prelude::*;


#[derive(Clone, Copy)]
pub struct Potion {
    _opaque_type : u8
}


unsafe impl DFValue for Potion {}
