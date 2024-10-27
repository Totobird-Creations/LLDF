use crate::prelude::*;


#[derive(Clone, Copy)]
pub struct Location {
    _opaque_type : u8
}


unsafe impl DFValue for Location {}
