use crate::prelude::*;


#[derive(Clone)]
pub struct RGB {
    _opaque_type : u8
}


unsafe impl DFValue for RGB {}
