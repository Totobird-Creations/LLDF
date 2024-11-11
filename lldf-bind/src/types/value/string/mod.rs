mod string;
pub use string::String;
mod text;
pub use text::Text;
mod colour;
#[cfg(any(not(feature = "en_us"), doc))]
pub use colour::Colour;
#[cfg(any(feature = "en_us", doc))]
pub use colour::Colour as Color;
mod uuid;
pub use uuid::UUID;


use crate::prelude::*;
use crate::bind::DFOpaqueValue;
