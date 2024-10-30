use crate::prelude::*;


#[derive(Clone)]
pub struct Particle {
    _opaque_type : u8
}


unsafe impl DFValue for Particle {}
