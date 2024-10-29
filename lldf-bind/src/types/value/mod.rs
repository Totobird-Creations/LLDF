pub mod string;
pub mod text;
pub mod number;
pub mod location;
pub mod vector;
pub mod sound;
pub mod particle;
pub mod potion;
pub mod item;
pub mod colour;
pub mod list;
pub mod dict;

use crate::prelude::*;


pub unsafe trait DFValue : Clone { }

unsafe impl<T : DFValue> DFValue for &T { }
