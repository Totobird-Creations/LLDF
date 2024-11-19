//! Reimplementation of the necessary parts of the Rust core library.


pub mod array;
pub mod clone;
pub mod cmp;
pub mod convert;
pub mod float;
pub mod int;
pub mod intrinsics;
pub mod macros;
pub mod marker;
pub mod mem;
pub mod ops;
pub mod option;
pub mod panic;
pub mod ptr;
pub mod slice;
pub mod str;
pub mod string;


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

#[rustc_builtin_macro]
pub macro stringify($($t:tt)*) {
    /* compiler built-in */
}


pub mod prelude {
    pub use super::clone;
    pub use super::clone::Clone;
    pub use super::cmp::{ PartialEq, PartialOrd, Ordering };
    pub use super::convert::{ Into, From, AsRef, AsMut };
    pub use super::macros::derive;
    pub use super::marker;
    pub use super::marker::Copy;
    pub use super::option::Option::{ self, Some, None };
    pub use super::string::{ ToString, FromStringUnchecked };
    pub use super::{
        include, include_str, include_bytes,
        concat, stringify,
        compile_error
    };
    pub use lldf_bind_proc::compile_warning;
}
