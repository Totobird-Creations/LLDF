use super::super::marker::Sized;


#[lang = "index"]
pub trait Index<Idx : ?Sized> {
    #[rustc_diagnostic_item = "IndexOutput"]
    type Output : ?Sized;
    fn index(&self, index : Idx) -> &Self::Output;
}
