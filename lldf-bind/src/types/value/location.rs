use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


#[derive(Clone)]
pub struct Location {
    _opaque_type : u8
}

impl Location {

    #[lldf_bind_proc::dfdoc(SetVariable/SetAllCoords { CoordinateType = PlotCoordinate })]
    #[inline(always)]
    pub fn new<F0 : Into<Float>, F1 : Into<Float>, F2 : Into<Float>, F3 : Into<Float>, F4 : Into<Float>>(x : F0, y : F1, z : F2, pitch : F3, yaw : F4) -> Self { unsafe {
        DF_ACTION__SetVariable_SetAllCoords_CoordinateType_PlotCoordinate(x.into(), y.into(), z.into(), pitch.into(), yaw.into())
    } }

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

    // TODO: raycast

}


unsafe impl DFValue for Location {
    #[inline]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe {
        transmute_unchecked(self._opaque_type.clone())
    } }
}


extern "C" {

    fn DF_ACTION__SetVariable_SetAllCoords_CoordinateType_PlotCoordinate( x : Float, y : Float, z : Float, pitch : Float, yaw : Float ) -> Location;

}
