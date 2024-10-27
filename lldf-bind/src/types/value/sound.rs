use crate::prelude::*;


#[derive(Clone, Copy)]
pub struct Sound {
    _opaque_type : u8
}


unsafe impl DFValue for Sound {}
