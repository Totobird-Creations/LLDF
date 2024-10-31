#[lang = "not"]
pub trait Not {
    type Output;
    #[must_use]
    fn not(self) -> Self::Output;
}
