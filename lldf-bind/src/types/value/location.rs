use crate::prelude::*;


#[derive(Clone)]
pub struct Location {
    _opaque_type : u8
}


unsafe impl DFValue for Location {}
