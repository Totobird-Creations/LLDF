mod string;
pub use string::String;
mod text;
pub use text::Text;
mod colour;
#[cfg(any(not(feature = "en_us"), doc))]
#[doc(cfg(not(feature = "en_us")))]
pub use colour::Colour;
#[cfg(any(feature = "en_us", doc))]
#[doc(cfg(feature = "en_us"))]
pub use colour::Colour as Color;
mod uuid;
pub use uuid::UUID;


use crate::prelude::*;
use crate::bind::DFOpaqueValue;
