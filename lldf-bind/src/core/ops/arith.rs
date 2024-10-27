#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[rustc_diagnostic_item = "add"]
    fn add(self, rhs : Rhs) -> Self::Output;
}
