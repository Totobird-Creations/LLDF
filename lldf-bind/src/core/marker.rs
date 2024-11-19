use super::macros::derive;
use super::clone::Clone;


#[doc(alias = "?", alias = "?Sized")]
#[lang = "sized"]
#[diagnostic::on_unimplemented(
    message = "the size for values of type `{Self}` cannot be known at compilation time",
    label = "doesn't have a size known at compile-time"
)]
#[fundamental]
#[rustc_specialization_trait]
#[rustc_deny_explicit_impl(implement_via_object = false)]
#[rustc_coinductive]
pub trait Sized {}

#[lang = "unsize"]
#[rustc_deny_explicit_impl(implement_via_object = false)]
pub trait Unsize<T : ?Sized> {}

#[lang = "copy"]
#[rustc_unsafe_specialization_marker]
#[rustc_diagnostic_item = "Copy"]
pub trait Copy : Clone {}
#[rustc_builtin_macro]
#[allow_internal_unstable(core_intrinsics, derive_clone_copy)]
pub macro Copy($item:item) {
    /* compiler built-in */
}

#[lang = "destruct"]
#[rustc_on_unimplemented(message = "can't drop `{Self}`", append_const_msg)]
#[rustc_deny_explicit_impl(implement_via_object = false)]
pub trait Destruct {}

#[lang = "freeze"]
pub unsafe auto trait Freeze {}

#[lang = "phantom_data"]
#[derive(Clone, Copy)]
pub struct PhantomData<T : ?Sized>;

#[lang = "sync"]
pub unsafe auto trait Sync {}

#[lang = "unpin"]
pub auto trait Unpin {}


#[lang = "tuple_trait"]
#[diagnostic::on_unimplemented(message = "`{Self}` is not a tuple")]
#[rustc_deny_explicit_impl(implement_via_object = false)]
pub trait Tuple {}
