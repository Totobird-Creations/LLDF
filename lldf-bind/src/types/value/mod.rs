mod string;
pub use string::*;
mod number;
pub use number::*;
mod location;
pub use location::Location;
mod vector;
pub use vector::Vector;
mod sound;
pub use sound::Sound;
mod particle;
pub use particle::Particle;
mod potion;
pub use potion::Potion;
mod item;
pub use item::Item;
mod list;
pub use list::List;
mod dict;
pub use dict::Dict;

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
