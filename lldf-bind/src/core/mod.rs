//! Reimplementation of the necessary parts of the Rust core library.


pub mod clone;
pub mod convert;
pub mod float;
pub mod int;
pub mod macros;
pub mod marker;
pub mod mem;
pub mod ops;
pub mod ptr;
pub mod str;


#[rustc_builtin_macro]
pub macro include($file:expr $(,)?) {
    /* compiler built-in */
}
#[rustc_builtin_macro]
pub macro include_str($file:expr $(,)?) {
    /* compiler built-in */
}
#[rustc_builtin_macro]
pub macro include_bytes($file:expr $(,)?) {
    /* compiler built-in */
}

#[rustc_builtin_macro]
pub macro concat($($e:expr),* $(,)?) {
    /* compiler built-in */
}

#[rustc_builtin_macro]
pub macro compile_error(&msg:expr $(,)?) {
    /* compiler built-in */
}


pub mod prelude {
    pub use super::clone;
    pub use super::clone::Clone;
    pub use super::convert::{ Into, From, AsRef, AsMut };
    pub use super::macros::derive;
    pub use super::marker;
    pub use super::marker::Copy;
    pub use super::{ include, include_str, include_bytes };
    pub use super::concat;
    pub use super::compile_error;
    pub use lldf_bind_proc::compile_warning;
}
