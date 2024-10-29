use super::super::marker::Sized;


#[lang = "index"]
pub trait Index<Idx : ?Sized> {
    #[rustc_diagnostic_item = "IndexOutput"]
    type Output : ?Sized;
    fn index(&self, index : Idx) -> &Self::Output;
}

#[lang = "index_mut"]
pub trait IndexMut<Idx : ?Sized> : Index<Idx> {
    #[rustc_diagnostic_item = "IndexMutOutput"]
    fn index_mut(&mut self, index : Idx) -> &mut <Self as Index<Idx>>::Output;
}
