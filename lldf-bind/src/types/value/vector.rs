use crate::prelude::*;


#[derive(Clone)]
pub struct Vector {
    _opaque_type : u8
}


unsafe impl DFValue for Vector {}
