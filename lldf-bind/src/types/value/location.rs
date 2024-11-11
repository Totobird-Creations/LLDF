use crate::prelude::*;
use crate::bind::DFOpaqueValue;


#[derive(Clone)]
pub struct Location {
    _opaque_type : u8
}

impl Location {

    // TODO: new

    // TODO: with_x

    // TODO: with_y

    // TODO: with_z

    // TODO: with_pos

    // TODO: with_pitch

    // TODO: with_yaw

    // TODO: with_rot

    // TODO: with_dir

    // TODO: with_facing

    // TODO: x

    // TODO: y

    // TODO: z

    // TODO: yaw

    // TODO: pitch

    // TODO: shift_axis

    // TODO: shiftv_axis

    // TODO: shift_dir

    // TODO: shiftv_dir

    // TODO: move_toward

    // TODO: dir

    // TODO: shift_rot

    // TODO: align

    // TODO: distance

    // TODO: midpoint

    // TODO: random

    // TODO: delta

    // TODO: noise_simplex

    // TODO: noise_perlin

    // TODO: noise_voronoi

    // TODO: noise_worley

    // TODO: noise_value0

    // TODO: noise_gradient

    // TODO: noise_cellular

    // TODO: noise_value1

}


unsafe impl DFValue for Location {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        DF_TRANSMUTE__Opaque(self)
    } }
}


extern "C" {

    fn DF_TRANSMUTE__Opaque( from : Location ) -> DFOpaqueValue;

}
