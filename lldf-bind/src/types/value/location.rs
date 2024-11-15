use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


/// A location in the plot.
/// - A position of `<0.0, 0.0, 0.0>` represents the north-west-bottom corner of the plot.
/// - A pitch of `0.0` is forward, `PI` is down, `-PI` is up.
/// - A yaw of `0.0` is south, `PI / 2.0` is west, `PI` is north, `-PI / 2.0` is east.
pub struct Location {
    _opaque_type : u64
}

impl Clone for Location {
    #[inline(always)]
    fn clone(&self) -> Self { unsafe { transmute_unchecked(self._opaque_type) } }
}

impl Location {

    #[lldf_bind_proc::dfdoc(SetVariable/SetAllCoords { CoordinateType = PlotCoordinate })]
    #[inline(always)]
    pub fn new<F0 : Into<Float>, F1 : Into<Float>, F2 : Into<Float>, F3 : Into<Float>, F4 : Into<Float>>(x : F0, y : F1, z : F2, pitch_rad : F3, yaw_rad : F4) -> Self { unsafe {
        DF_ACTION__SetVariable_SetAllCoords_CoordinateType_PlotCoordinate(x.into(), y.into(), z.into(), pitch_rad.into().to_degrees(), yaw_rad.into().to_degrees())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetCoord { CoordinateType = PlotCoordinate, Coordinate = X })]
    #[inline(always)]
    pub fn x(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_X(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCoord { CoordinateType = PlotCoordinate, Coordinate = X })]
    #[inline(always)]
    pub fn with_x<F : Into<Float>>(&self, x : F) -> Location { unsafe {
        DF_ACTION__SetVariable_SetCoord_CoordinateType_PlotCoordinate_Coordinate_X(self.to_opaque(), x.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ShiftOnAxis { Coordinate = X })]
    #[inline(always)]
    pub fn shift_x<F : Into<Float>>(&self, dx : F) -> Location { unsafe {
        DF_ACTION__SetVariable_ShiftOnAxis_Coordinate_X(self.to_opaque(), dx.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetCoord { CoordinateType = PlotCoordinate, Coordinate = Y })]
    #[inline(always)]
    pub fn y(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Y(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCoord { CoordinateType = PlotCoordinate, Coordinate = Y })]
    #[inline(always)]
    pub fn with_y<F : Into<Float>>(&self, y : F) -> Location { unsafe {
        DF_ACTION__SetVariable_SetCoord_CoordinateType_PlotCoordinate_Coordinate_Y(self.to_opaque(), y.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ShiftOnAxis { Coordinate = Y })]
    #[inline(always)]
    pub fn shift_y<F : Into<Float>>(&self, dy : F) -> Location { unsafe {
        DF_ACTION__SetVariable_ShiftOnAxis_Coordinate_Y(self.to_opaque(), dy.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetCoord { CoordinateType = PlotCoordinate, Coordinate = Z })]
    #[inline(always)]
    pub fn z(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Z(self.to_opaque())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCoord { CoordinateType = PlotCoordinate, Coordinate = Z })]
    #[inline(always)]
    pub fn with_z<F : Into<Float>>(&self, z : F) -> Location { unsafe {
        DF_ACTION__SetVariable_SetCoord_CoordinateType_PlotCoordinate_Coordinate_Z(self.to_opaque(), z.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ShiftOnAxis { Coordinate = Z })]
    #[inline(always)]
    pub fn shift_z<F : Into<Float>>(&self, dz : F) -> Location { unsafe {
        DF_ACTION__SetVariable_ShiftOnAxis_Coordinate_Z(self.to_opaque(), dz.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetCoord { CoordinateType = PlotCoordinate })]
    #[inline(always)]
    pub fn xyz(&self) -> (Float, Float, Float) { unsafe {
        let x = DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_X(self.to_opaque());
        let y = DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Y(self.to_opaque());
        let z = DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Z(self.to_opaque());
        (x, y, z)
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetCoord { CoordinateType = PlotCoordinate })]
    #[inline(always)]
    pub fn xyzv(&self) -> Vector<3> { unsafe {
        let x = DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_X(self.to_opaque());
        let y = DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Y(self.to_opaque());
        let z = DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Z(self.to_opaque());
        Vector::new([x, y, z])
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetAllCoords)]
    #[inline(always)]
    pub fn with_xyz<F0 : Into<Float>, F1 : Into<Float>, F2 : Into<Float>>(&self, x : F0, y : F1, z : F2) -> Location { unsafe {
        DF_ACTION__SetVariable_SetAllCoords_CoordinateType_PlotCoordinate(self.to_opaque(), x.into(), y.into(), z.into())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetAllCoords)]
    #[inline(always)]
    pub fn with_xyzv<V : AsRef<Vector<3>>>(&self, xyz : V) -> Location { unsafe {
        let xyz = xyz.as_ref();
        DF_ACTION__SetVariable_SetAllCoords_CoordinateType_PlotCoordinate(self.to_opaque(), xyz.x(), xyz.y(), xyz.z())
    } }

    // TODO: shift_xyz

    // TODO: shiftv_xyz, Add<Vector<3>>

    #[lldf_bind_proc::dfdoc(SetVariable/GetCoord { CoordinateType = PlotCoordinate, Coordinate = Pitch })]
    /// ##### Note:
    /// - The return value is in **radians**.
    #[inline(always)]
    pub fn pitch(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Pitch(self.to_opaque()).to_radians()
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCoord { CoordinateType = PlotCoordinate, Coordinate = Pitch })]
    /// ##### Note:
    /// - The angle value is in **radians**.
    #[inline(always)]
    pub fn with_pitch<F : Into<Float>>(&self, pitch_rad : F) -> Location { unsafe {
        DF_ACTION__SetVariable_SetCoord_CoordinateType_PlotCoordinate_Coordinate_Pitch(self.to_opaque(), pitch_rad.into().to_degrees())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ShiftRotation { RotationAxis = Pitch })]
    #[inline(always)]
    pub fn shift_pitch<F : Into<Float>>(&self, dpitch : F) -> Location { unsafe {
        DF_ACTION__SetVariable_ShiftRotation_RotationAxis_Pitch(self.to_opaque(), dpitch.into().to_degrees())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/GetCoord { CoordinateType = PlotCoordinate, Coordinate = Yaw })]
    /// ##### Note:
    /// - The return value is in **radians**.
    #[inline(always)]
    pub fn yaw(&self) -> Float { unsafe {
        DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Yaw(self.to_opaque()).to_radians()
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/SetCoord { CoordinateType = PlotCoordinate, Coordinate = Yaw })]
    /// ##### Note:
    /// - The angle value is in **radians**.
    #[inline(always)]
    pub fn with_yaw<F : Into<Float>>(&self, yaw_rad : F) -> Location { unsafe {
        DF_ACTION__SetVariable_SetCoord_CoordinateType_PlotCoordinate_Coordinate_Yaw(self.to_opaque(), yaw_rad.into().to_degrees())
    } }

    #[lldf_bind_proc::dfdoc(SetVariable/ShiftRotation { RotationAxis = Yaw })]
    #[inline(always)]
    pub fn shift_yaw<F : Into<Float>>(&self, dyaw : F) -> Location { unsafe {
        DF_ACTION__SetVariable_ShiftRotation_RotationAxis_Yaw(self.to_opaque(), dyaw.into().to_degrees())
    } }

    // TODO: rot

    #[lldf_bind_proc::dfdoc(SetVariable/SetAllCoords)]
    /// ##### Note:
    /// - The angle values are in **radians**.
    #[inline(always)]
    pub fn with_rot<F0 : Into<Float>, F1 : Into<Float>>(&self, pitch_rad : F0, yaw_rad : F1) -> Location { unsafe {
        DF_ACTION__SetVariable_SetAllCoords_CoordinateType_PlotCoordinate(self.to_opaque(), Item::air(), Item::air(), Item::air(), pitch_rad.into().to_degrees(), yaw_rad.into().to_degrees())
    } }

    // TODO: with_rotv

    // TODO: shift_rot

    // TODO: shiftv_rot

    // TODO: direction

    // TODO: with_look_to

    // TODO: with_facing

    // TODO: shift_forward

    // TODO: shift_up

    // TODO: shift_right

    // TODO: shift_relative

    // TODO: shiftv_relative

    // TODO: move_toward

    // TODO: align

    // TODO: distance

    // TODO: midpoint

    #[lldf_bind_proc::dfdoc(SetVariable/RandomLoc)]
    #[inline(always)]
    pub fn random_in<L : AsRef<Location>>(&self, other : L) -> Location { unsafe {
        let other = other.as_ref();
        DF_ACTION__SetVariable_RandomLoc(self.to_opaque(), other.to_opaque())
    } }

    // TODO: delta, Sub<Location>

    // TODO: noise_simplex

    // TODO: noise_perlin

    // TODO: noise_voronoi

    // TODO: noise_worley

    // TODO: noise_value0

    // TODO: noise_gradient

    // TODO: noise_cellular

    // TODO: noise_value1

    // TODO: raycast_unchecked

    // TODO: raycast

    // TODO: raycastv_unchecked

    // TODO: raycastv

}


unsafe impl DFValue for Location {
    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe { transmute_unchecked(self._opaque_type) } }
}


extern "C" {

    fn DF_ACTION__SetVariable_SetAllCoords_CoordinateType_PlotCoordinate( ... ) -> Location;
    fn DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_X( location : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Y( location : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Z( location : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Pitch( location : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_GetCoord_CoordinateType_PlotCoordinate_Coordinate_Yaw( location : DFOpaqueValue ) -> Float;
    fn DF_ACTION__SetVariable_SetCoord_CoordinateType_PlotCoordinate_Coordinate_X( location : DFOpaqueValue, x : Float ) -> Location;
    fn DF_ACTION__SetVariable_SetCoord_CoordinateType_PlotCoordinate_Coordinate_Y( location : DFOpaqueValue, y : Float ) -> Location;
    fn DF_ACTION__SetVariable_SetCoord_CoordinateType_PlotCoordinate_Coordinate_Z( location : DFOpaqueValue, z : Float ) -> Location;
    fn DF_ACTION__SetVariable_SetCoord_CoordinateType_PlotCoordinate_Coordinate_Pitch( location : DFOpaqueValue, pitch : Float ) -> Location;
    fn DF_ACTION__SetVariable_SetCoord_CoordinateType_PlotCoordinate_Coordinate_Yaw( location : DFOpaqueValue, yaw : Float ) -> Location;
    fn DF_ACTION__SetVariable_ShiftOnAxis_Coordinate_X( location : DFOpaqueValue, dx : Float ) -> Location;
    fn DF_ACTION__SetVariable_ShiftOnAxis_Coordinate_Y( location : DFOpaqueValue, dy : Float ) -> Location;
    fn DF_ACTION__SetVariable_ShiftOnAxis_Coordinate_Z( location : DFOpaqueValue, dz : Float ) -> Location;
    fn DF_ACTION__SetVariable_ShiftRotation_RotationAxis_Pitch( location : DFOpaqueValue, dpitch : Float ) -> Location;
    fn DF_ACTION__SetVariable_ShiftRotation_RotationAxis_Yaw( location : DFOpaqueValue, dyaw : Float ) -> Location;

    fn DF_ACTION__SetVariable_RandomLoc( a : DFOpaqueValue, b : DFOpaqueValue ) -> Location;

}
