use super::marker::Sized;


#[rustc_diagnostic_item = "Into"]
pub trait Into<T> : Sized {
    #[must_use]
    fn into(self) -> T;
}

impl<T> Into<T> for T {
    fn into(self) -> T { self }
}


#[rustc_diagnostic_item = "From"]
pub trait From<T> : Sized {
    #[must_use]
    fn from(value : T) -> Self;
}
