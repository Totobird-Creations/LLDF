mod string;
pub use string::*;
mod number;
pub use number::*;
mod location;
pub use location::*;
mod sound;
pub use sound::*;
mod particle;
pub use particle::*;
mod potion;
pub use potion::*;
mod item;
pub use item::*;
mod list;
pub use list::*;
mod dict;
pub use dict::*;

mod vector;
pub use vector::*;
mod matrix;
pub use matrix::*;

use crate::prelude::*;
use crate::bind::DFOpaqueValue;


pub unsafe trait DFValue : Clone {
    unsafe fn to_opaque(self) -> DFOpaqueValue;
}

unsafe impl<T : DFValue> DFValue for &T {
    #[inline(always)]
    unsafe fn to_opaque(self) -> DFOpaqueValue {
        self.clone().to_opaque()
    }
}
