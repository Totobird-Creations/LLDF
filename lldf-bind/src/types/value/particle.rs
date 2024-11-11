use crate::prelude::*;
use crate::bind::DFOpaqueValue;


/// A particle effect with customisable parameters.
#[derive(Clone)]
pub struct Particle {
    _opaque_type : u8
}


impl Particle {

    // TODO: kind

    // TODO: amount

    // TODO: with_amount

    // TODO: spread

    // TODO: with_spread

    // TODO: size

    // TODO: with_size

    // TODO: material

    // TODO: with_material

    // TODO: colour

    // TODO: with_colour

    // TODO: opacity

    // TODO: with_opacity

    // TODO: motion

    // TODO: with_motion

    // TODO: roll

    // TODO: with_roll

    // TODO: fade_colour

    // TODO: with_fade_colour

}


unsafe impl DFValue for Particle {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


extern "C" {

    fn DF_TRANSMUTE__Opaque( from : Particle ) -> DFOpaqueValue;

}


include!("../../bind/particle.rs");

