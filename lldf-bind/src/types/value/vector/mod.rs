use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


mod other;
mod one;
mod two;
mod three;
mod four;


/// A vector with some number of lanes. Used for representing directions, motions, or offsets.
pub struct Vector<const LANES : usize> { // TODO: Clone
    inner : List<Float>
}
#[doc(hidden)]
pub trait _VectorMethods<const LANES : usize> : DFValue {

    fn new(lanes : [Float; LANES]) -> Vector<LANES>;

    fn splat<F : Into<Float>>(lanes : F) -> Vector<LANES>;

    // TODO: with_lane_unchecked

    // TODO: with_lane (with bounds check)

    unsafe fn lane_unchecked<U : Into<UInt>>(&self, lane : U) -> Float;

    // TODO: lane (with bounds check)

    // TODO: length

    // TODO: length_squared

    // TODO: with_length

    // TODO: Add

    // TODO: Sub

    // TODO: Mul

    // TODO: Div

    // TODO: align

    // TODO: rotate_x

    // TODO: rotate_y

    // TODO: rotate_z

    // TODO: rotate

    // TODO: reflect

    // TODO: dot

    // TODO: to_array

    // TODO: to_list

    // TODO: normalise

    // TODO: project_on

    // TODO: reject_from

    // TODO: midpoint

    // TODO: angle_between


    #[doc(hidden)]
    fn clone(&self) -> Self;

}
impl<const LANES : usize> Clone for Vector<LANES> {
    fn clone(&self) -> Self { _VectorMethods::<LANES>::clone(self) }
}

unsafe impl<const LANES : usize> DFValue for Vector<LANES> {
    #[inline(always)]
    unsafe fn to_opaque(&self) -> DFOpaqueValue { unsafe {
        self.inner.to_opaque()
    } }
}
