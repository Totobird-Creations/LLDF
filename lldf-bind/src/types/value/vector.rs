use crate::prelude::*;


#[derive(Clone, Copy)]
pub struct Vector {
    _opaque_type : u8
}


unsafe impl DFValue for Vector {}
