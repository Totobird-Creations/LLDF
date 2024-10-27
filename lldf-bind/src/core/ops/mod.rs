mod arith;
pub use arith::Add;
mod index;
pub use index::Index;


use super::marker::Sized;


#[lang = "receiver"]
trait Receiver {}
impl<T: ?Sized> Receiver for &T {}
impl<T: ?Sized> Receiver for &mut T {}
