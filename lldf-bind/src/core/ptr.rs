use super::clone::Clone;
use super::marker::{ Copy, Sized };


#[allow(unconditional_recursion)]
#[lang = "drop_in_place"]
#[rustc_diagnostic_item = "ptr_drop_in_place"]
pub unsafe fn drop_in_place<T : ?Sized>(_ : *mut T) {
    loop { /* compiler built-in */ }
}

impl<T> Clone for &T { fn clone(&self) -> Self { *self } }
impl<T> Copy for &T { }
