use super::super::marker::Sized;


#[lang = "deref"]
#[rustc_diagnostic_item = "Deref"]
pub trait Deref {
    #[rustc_diagnostic_item = "deref_target"]
    type Target : ?Sized;
    #[rustc_diagnostic_item = "deref_method"]
    fn deref(&self) -> &Self::Target;
}

#[lang = "deref_mut"]
pub trait DerefMut : Deref {
    #[rustc_diagnostic_item = "deref_mut_method"]
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target;
}
