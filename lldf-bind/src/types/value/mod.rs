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
    unsafe fn to_opaque(&self) -> DFOpaqueValue;
    fn to_string(&self) -> String;
    fn to_text(&self) -> Text;
}
impl<T : DFValue> From<T> for String {
    fn from(value : T) -> Self { value.to_string() }
}
impl<T : DFValue> From<T> for Text {
    fn from(value : T) -> Self { value.to_text() }
}
impl<T : DFValue> ToString for T {
    fn to_string(&self) -> String { self.to_string() }
}


pub unsafe trait DFSortableValue : DFValue { }
unsafe impl DFSortableValue for UInt { }
unsafe impl DFSortableValue for Int { }
unsafe impl DFSortableValue for Float { }
unsafe impl DFSortableValue for String { }
