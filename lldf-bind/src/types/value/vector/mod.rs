use crate::prelude::*;
use crate::bind::DFOpaqueValue;
use crate::core::mem::transmute_unchecked;


mod one;
mod two;
mod three;
mod four;
mod other;


/// A vector with X, Y, and Z values. Used for representing directions, motions, or offsets.
#[derive(Clone)]
pub struct Vector<const LANES : usize> {
    inner : List<Float>
}
#[doc(hidden)]
pub trait _VectorMethods<const LANES : usize> {

    fn new(lanes : [Float; LANES]) -> Vector<LANES>;

    // TODO: splat

    // TODO: with_lane

    fn lane<U : Into<UInt>>(&self, lane : U) -> Float;

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

    // TODO: ext

    // TODO: trunc

    // TODO: normalise

    // TODO: project_on

    // TODO: reject_from

    // TODO: midpoint

    // TODO: angle_between

    // TODO: 

}

unsafe impl<const LANES : usize> DFValue for Vector<LANES> {
    #[inline]
    unsafe fn to_opaque(self) -> DFOpaqueValue { unsafe {
        transmute_unchecked(self)
    } }
}
