mod arith;
pub use arith::*;
mod bit;
pub use bit::*;
mod deref;
pub use deref::*;
mod function;
pub use function::*;
mod index;
pub use index::*;


use super::marker::Sized;


#[lang = "legacy_receiver"]
trait Receiver {}
impl<T: ?Sized> Receiver for &T {}
impl<T: ?Sized> Receiver for &mut T {}
