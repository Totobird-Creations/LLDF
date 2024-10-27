use super::marker::Sized;


#[lang = "clone"]
#[rustc_diagnostic_item = "Clone"]
#[rustc_trivial_field_reads]
pub trait Clone : Sized {
    fn clone(&self) -> Self;
    #[inline] fn clone_from(&mut self, source : &Self) {
        *self = source.clone()
    }
}
#[rustc_builtin_macro]
#[allow_internal_unstable(core_intrinsics, derive_clone_copy)]
pub macro Clone($item:item) {
    /* compiler built-in */
}


#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct AssertParamIsClone<T : Clone + ?Sized> {
    _field: crate::marker::PhantomData<T>,
}
